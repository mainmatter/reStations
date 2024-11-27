use anyhow::{anyhow, Context};
use clap::{Parser, Subcommand};
use cruet::case::snake::to_snake_case;
use guppy::{graph::PackageGraph, MetadataCommand};
use liquid::Template;
use restations_cli::util::ui::UI;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
static BLUEPRINTS_DIR: include_dir::Dir =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/blueprints");

#[tokio::main]
async fn main() {
    cli().await;
}

#[derive(Parser)]
#[command(author, version, about = "A CLI tool to generate project files.", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true, help = "Disable colored output.")]
    no_color: bool,

    #[arg(long, global = true, help = "Disable debug output.")]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate a middleware")]
    Middleware {
        #[arg(help = "The name of the middleware.")]
        name: String,
    },
    #[command(about = "Generate a controller")]
    Controller {
        #[arg(help = "The name of the controller.")]
        name: String,
    },
    #[command(about = "Generate a test for a controller")]
    ControllerTest {
        #[arg(help = "The name of the controller.")]
        name: String,
    },
}

#[allow(missing_docs)]
pub async fn cli() {
    let cli = Cli::parse();
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    let mut ui = UI::new(&mut stdout, &mut stderr, !cli.no_color, !cli.quiet);

    match cli.command {
        Commands::Middleware { name } => {
            ui.info("Generating middleware…");
            match generate_middleware(name).await {
                Ok(file_name) => ui.success(&format!("Generated middleware {}.", &file_name)),
                Err(e) => ui.error("Could not generate middleware!", e),
            }
        }
        Commands::Controller { name } => {
            ui.info("Generating controller…");
            match generate_controller(name.clone()).await {
                Ok(file_name) => {
                    ui.success(&format!("Generated controller {}.", &file_name));
                    ui.info(
                        "Do not forget to route the controller's actions in ./web/src/routes.rs!",
                    );
                }
                Err(e) => ui.error("Could not generate controller!", e),
            }
            ui.info("Generating test for controller…");
            match generate_controller_test(name).await {
                Ok(file_name) => {
                    ui.success(&format!("Generated test for controller {}.", &file_name))
                }
                Err(e) => ui.error("Could not generate test for controller!", e),
            }
        }
        Commands::ControllerTest { name } => {
            ui.info("Generating test for controller…");
            match generate_controller_test(name).await {
                Ok(file_name) => {
                    ui.success(&format!("Generated test for controller {}.", &file_name))
                }
                Err(e) => ui.error("Could not generate test for controller!", e),
            }
        }
    }
}

async fn generate_middleware(name: String) -> Result<String, anyhow::Error> {
    let name = to_snake_case(&name).to_lowercase();

    let template = get_liquid_template("middleware/file.rs")?;
    let variables = liquid::object!({
        "name": name
    });
    let output = template
        .render(&variables)
        .context("Failed to render Liquid template")?;

    let file_path = format!("./web/src/middlewares/{}.rs", name);
    create_project_file(&file_path, output.as_bytes())?;
    append_to_project_file(
        "./web/src/middlewares/mod.rs",
        &format!("pub mod {};", name),
    )?;

    Ok(file_path)
}

async fn generate_controller(name: String) -> Result<String, anyhow::Error> {
    let name = to_snake_case(&name).to_lowercase();

    let template = get_liquid_template("controller/minimal/controller.rs")?;
    let variables = liquid::object!({
        "name": name,
    });
    let output = template
        .render(&variables)
        .context("Failed to render Liquid template")?;

    let file_path = format!("./web/src/controllers/{}.rs", name);
    create_project_file(&file_path, output.as_bytes())?;
    append_to_project_file(
        "./web/src/controllers/mod.rs",
        &format!("pub mod {};", name),
    )?;

    Ok(file_path)
}

async fn generate_controller_test(name: String) -> Result<String, anyhow::Error> {
    let name = to_snake_case(&name).to_lowercase();
    let macros_crate_name = get_member_package_name("macros")?;
    let macros_crate_name = to_snake_case(&macros_crate_name);
    let has_db = has_db();

    let template = get_liquid_template("controller/minimal/test.rs")?;
    let variables = liquid::object!({
        "name": name,
        "macros_crate_name": macros_crate_name,
        "has_db": has_db,
    });
    let output = template
        .render(&variables)
        .context("Failed to render Liquid template")?;

    let file_path = format!("./web/tests/api/{name}_test.rs");
    create_project_file(&file_path, output.as_bytes())?;
    append_to_project_file("./web/tests/api/main.rs", &format!("mod {name}_test;"))?;

    Ok(file_path)
}

fn get_liquid_template(path: &str) -> Result<Template, anyhow::Error> {
    let blueprint = BLUEPRINTS_DIR
        .get_file(path)
        .context(format!("Failed to get blueprint {}!", path))?;
    let template_source = blueprint
        .contents_utf8()
        .context(format!("Failed to read blueprint {}!", path))?;
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template_source)
        .context("Failed to parse blueprint as Liquid template")?;

    Ok(template)
}

fn create_project_file(path: &str, contents: &[u8]) -> Result<(), anyhow::Error> {
    let mut file = File::create(path).context(format!(r#"Could not create file "{}""#, path))?;
    file.write_all(contents)
        .context(format!(r#"Could not write file "{}""#, path))?;

    Ok(())
}

fn append_to_project_file(path: &str, contents: &str) -> Result<(), anyhow::Error> {
    let file_contents =
        fs::read_to_string(path).context(format!(r#"Could not read file "{}"!"#, path))?;
    let file_contents = file_contents.trim();

    let mut options = OpenOptions::new();
    options.write(true);

    if file_contents.is_empty() {
        options.truncate(true);
    } else {
        options.append(true);
    }

    let mut file = options
        .open(path)
        .context(format!(r#"Could not open file "{}"!"#, path))?;

    writeln!(file, "{}", contents).context(format!(r#"Failed to append to file "{}"!"#, path))?;

    Ok(())
}

fn has_db() -> bool {
    get_member_package_name("db").is_ok()
}

fn get_member_package_name(path: &str) -> Result<String, anyhow::Error> {
    let mut cmd = MetadataCommand::new();
    let package_graph = PackageGraph::from_command(cmd.manifest_path("./Cargo.toml")).unwrap();
    let workspace = package_graph.workspace();
    for member in workspace.iter_by_path() {
        let (member_path, metadata) = member;
        if member_path == path {
            return Ok(String::from(metadata.name()));
        }
    }
    Err(anyhow!("Could not find workspace member at path: {}", path))
}

use anyhow::{anyhow, Context};
use clap::{Parser, Subcommand};
use guppy::{Version, VersionReq};
use restations_cli::util::ui::UI;
use restations_config::{load_config, parse_env, Config, Environment};
use std::path::PathBuf;
use std::process::{ExitCode, Stdio};
use tokio::io::{stdin, AsyncBufReadExt};

#[tokio::main]
async fn main() -> ExitCode {
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();

    let args = Cli::parse();
    let mut ui = UI::new(&mut stdout, &mut stderr, !args.no_color, !args.quiet);

    match cli(&mut ui, args).await {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            ui.error(e.to_string().as_str(), &e);
            ExitCode::FAILURE
        }
    }
}

#[derive(Parser)]
#[command(author, version, about = "A CLI tool to manage the project's database.", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true, help = "Choose the environment (development, test, production).", value_parser = parse_env, default_value = "development")]
    env: Environment,

    #[arg(long, global = true, help = "Disable colored output.")]
    no_color: bool,

    #[arg(long, global = true, help = "Disable debug output.")]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate query metadata to support offline compile-time verification")]
    Prepare,
}

#[allow(missing_docs)]
async fn cli(ui: &mut UI<'_>, cli: Cli) -> Result<(), anyhow::Error> {
    let config: Result<Config, anyhow::Error> = load_config(&cli.env);
    match config {
        Ok(config) => {
            match cli.command {
                Commands::Prepare => {
                    if let Err(e) = ensure_sqlx_cli_installed(ui).await {
                        return Err(e.context("Error ensuring sqlx-cli is installed!"));
                    }

                    let cargo = get_cargo_path().expect("Existence of CARGO env var is asserted by calling `ensure_sqlx_cli_installed`");

                    let mut sqlx_prepare_command = {
                        let mut cmd = tokio::process::Command::new(&cargo);

                        cmd.args(["sqlx", "prepare", "--", "--all-targets", "--all-features"]);

                        let cmd_cwd = db_package_root()
                            .context("Error finding the root of the db package!")?;
                        cmd.current_dir(cmd_cwd);

                        cmd.env("DATABASE_URL", &config.database.url);
                        cmd
                    };

                    let o = sqlx_prepare_command
                        .output()
                        .await
                        .context("Could not run {cargo} sqlx prepare!")?;
                    if !o.status.success() {
                        let error = anyhow!(String::from_utf8_lossy(&o.stdout).to_string()).context("Error generating query metadata. Are you sure the database is running and all migrations are applied?");
                        return Err(error);
                    }

                    ui.success("Query data written to db/.sqlx directory; please check this into version control.");
                    Ok(())
                }
            }
        }
        Err(e) => Err(e.context("Could not load config!")),
    }
}

fn get_cargo_path() -> Result<String, anyhow::Error> {
    std::env::var("CARGO")
        .map_err(|_| anyhow!("Please invoke me using Cargo, e.g.: `cargo db <ARGS>`"))
}

/// Ensure that the correct version of sqlx-cli is installed,
/// and install it if it isn't.
async fn ensure_sqlx_cli_installed(ui: &mut UI<'_>) -> Result<(), anyhow::Error> {
    /// The version of sqlx-cli required
    const SQLX_CLI_VERSION: &str = "0.8";
    let sqlx_version_req = VersionReq::parse(SQLX_CLI_VERSION)
        .expect("SQLX_CLI_VERSION value is not a valid semver version requirement.");

    /// Get the version of the current sqlx-cli installation, if any.
    async fn installed_sqlx_cli_version(cargo: &str) -> Result<Option<Version>, anyhow::Error> {
        /// The expected prefix of the version output of sqlx-cli >= 0.8
        const SQLX_CLI_VERSION_STRING_PREFIX: &str = "sqlx-cli-sqlx";
        /// The expected prefix of the version output of sqlx-cli < 0.8
        const SQLX_CLI_VERSION_STRING_PREFIX_OLD: &str = "cargo-sqlx";

        fn error_parsing_version() -> anyhow::Error {
            anyhow!(
                "Error parsing sqlx-cli version. Please install the \
                correct version manually using `cargo install sqlx-cli \
                --version ^{SQLX_CLI_VERSION} --locked`"
            )
        }

        let mut cargo_sqlx_command = {
            let mut cmd = tokio::process::Command::new(cargo);
            cmd.args(["sqlx", "--version"]);
            cmd
        };

        let out = cargo_sqlx_command.output().await?;
        if !out.status.success() {
            // Failed to run the command for some reason,
            // we conclude that sqlx-cli is not installed.
            return Ok(None);
        }

        let Ok(stdout) = String::from_utf8(out.stdout) else {
            return Err(error_parsing_version());
        };

        let Some(version) = stdout
            .strip_prefix(SQLX_CLI_VERSION_STRING_PREFIX)
            .or_else(|| stdout.strip_prefix(SQLX_CLI_VERSION_STRING_PREFIX_OLD))
            .map(str::trim)
        else {
            return Err(error_parsing_version());
        };

        let Ok(version) = Version::parse(version) else {
            return Err(error_parsing_version());
        };

        Ok(Some(version))
    }

    let cargo = get_cargo_path()?;

    let current_version = installed_sqlx_cli_version(&cargo).await?;
    if let Some(version) = &current_version {
        if sqlx_version_req.matches(version) {
            // sqlx-cli is already installed and of the correct version, nothing to do
            return Ok(());
        }
    }

    let curr_vers_msg = current_version
        .map(|v| format!("The currently installed version is {v}."))
        .unwrap_or_else(|| "sqlx-cli is currently not installed.".to_string());
    ui.info(&format!(
        "This command requires a version of sqlx-cli that is \
        compatible with version {SQLX_CLI_VERSION}, which is not installed yet. \
        {curr_vers_msg} \
        Would you like to install the latest compatible version now? [Y/n]"
    ));

    // Read user answer
    {
        let mut buf = String::new();
        let mut reader = tokio::io::BufReader::new(stdin());
        loop {
            reader.read_line(&mut buf).await?;
            let line = buf.to_ascii_lowercase();
            let line = line.trim_end();
            if matches!(line, "" | "y" | "yes") {
                ui.info("Starting installation of sqlx-cli...");
                break;
            } else if matches!(line, "n" | "no") {
                return Err(anyhow!("Installation of sqlx-cli canceled."));
            };
            ui.info("Please enter y or n");
            buf.clear();
        }
    }

    let mut cargo_install_command = {
        let mut cmd = tokio::process::Command::new(&cargo);
        cmd.args([
            "install",
            "sqlx-cli",
            "--version",
            &format!("^{SQLX_CLI_VERSION}"),
            "--locked",
            // Install unoptimized version,
            // making the process much faster.
            // sqlx-cli doesn't really need to be
            // performant anyway for our purposes
            "--debug",
        ]);
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        cmd
    };

    let mut child = cargo_install_command.spawn()?;

    let status = child.wait().await?;
    if !status.success() {
        return Err(anyhow!(
            "Something went wrong when installing sqlx-cli. Please check output"
        ));
    }

    match installed_sqlx_cli_version(&cargo).await {
        Ok(Some(v)) if sqlx_version_req.matches(&v) => {
            ui.success(&format!("Successfully installed sqlx-cli {v}"));
            Ok(())
        }
        Ok(Some(v)) => Err(anyhow!("Could not update sqlx cli. Current version: {v}")),
        Ok(None) => Err(anyhow!("sqlx-cli was not detected after installation")),
        Err(e) => Err(e),
    }
}

/// Find the root of the db package in the gerust workspace.
fn db_package_root() -> Result<PathBuf, anyhow::Error> {
    Ok(PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|e| anyhow!(e).context("This command needs to be invoked using cargo"))?,
    )
    .join("..")
    .join("db")
    .canonicalize()?)
}

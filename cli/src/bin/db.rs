use anyhow::{anyhow, Context};
use clap::{Parser, Subcommand};
use csv_async::{AsyncReaderBuilder, StringRecord, Trim};
use futures::stream::TryStreamExt;
use guppy::{Version, VersionReq};
use reqwest::Client;
use restations_cli::util::ui::UI;
use restations_config::DatabaseConfig;
use restations_config::{load_config, parse_env, Config, Environment};
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};
use sqlx::{ConnectOptions, Connection};
use std::path::PathBuf;
use std::process::{ExitCode, Stdio};
use tokio::{
    fs::{read_to_string, remove_file, File},
    io::{stdin, AsyncBufReadExt},
};
use tokio_stream::StreamExt;
use url::Url;

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
    #[command(about = "Drop the database")]
    Drop,
    #[command(about = "Create the database")]
    Create,
    #[command(about = "Synchronize the database with the source data")]
    Sync,
    #[command(about = "Generate query metadata to support offline compile-time verification")]
    Prepare,
}

#[allow(missing_docs)]
async fn cli(ui: &mut UI<'_>, cli: Cli) -> Result<(), anyhow::Error> {
    let config: Result<Config, anyhow::Error> = load_config(&cli.env);
    match config {
        Ok(config) => {
            match cli.command {
                Commands::Drop => {
                    ui.info(&format!("Dropping {} database…", &cli.env));
                    let db_name = drop(&config.database)
                        .await
                        .context("Could not drop database!")?;
                    ui.success(&format!("Dropped database {} successfully.", db_name));
                    Ok(())
                }
                Commands::Create => {
                    ui.info(&format!("Creating {} database…", &cli.env));
                    let db_name = create(&config.database)
                        .await
                        .context("Could not create database!")?;
                    ui.success(&format!("Created database {} successfully.", db_name));
                    Ok(())
                }
                Commands::Sync => {
                    ui.info(&format!("Synchronizing {} database…", &cli.env));
                    ui.indent();
                    let stations = sync(&config)
                        .await
                        .context("Could not synchronize database!");
                    ui.outdent();
                    let stations = stations?;
                    ui.success(&format!("{} stations synchronized.", stations));
                    Ok(())
                }
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

async fn drop(config: &DatabaseConfig) -> Result<String, anyhow::Error> {
    let db_config = get_db_config(config);
    let db_file_name = db_config.get_filename();
    remove_file(db_file_name)
        .await
        .context("Failed to remove database file!")?;
    Ok(db_file_name.to_string_lossy().to_string())
}

async fn create(config: &DatabaseConfig) -> Result<String, anyhow::Error> {
    let db_config = get_db_config(config);
    let db_file_name = db_config.get_filename();
    File::create_new(db_file_name)
        .await
        .context("Failed to create database file!")?;

    let mut connection = get_db_client(config).await;

    let db_package_root = db_package_root().context("Failed to get db package root!")?;
    let schema_file = PathBuf::from_iter([db_package_root, "schema.sql".into()]);
    let statements = read_to_string(schema_file)
        .await
        .expect("Could not read schema – make sure db/schema.sql exists!");

    sqlx::query(statements.as_str())
        .execute(&mut connection)
        .await
        .context("Failed to create schema!")?;

    Ok(db_file_name.to_string_lossy().to_string())
}

struct StationRecord {
    pub id: i64,
    pub name: String,
    pub uic: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub country: Option<String>,
}

async fn sync(config: &Config) -> Result<i32, anyhow::Error> {
    let client = Client::new();
    let response = client.get(config.source_data_file.clone()).send().await?;
    let stream = response
        .bytes_stream()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
    let reader = tokio_util::io::StreamReader::new(stream);

    let mut rdr = AsyncReaderBuilder::new()
        .trim(Trim::All)
        .delimiter(b';')
        .create_reader(reader);
    let mut records = rdr.records();

    let mut conn = get_db_client(&config.database).await;

    let mut i = 0;
    while let Some(record) = records.next().await {
        let record = record.context("Failed to read record from CSV file!")?;
        let station = prepare_station(record, i)?;

        // TODO: get name (all languages), country name (all languages), country code, postcode, city (all languages), display name (all languages), street (all languages)
        // localhost:8080/reverse?format=jsonv2&lat=48.876742&lon=2.358424&addressdetails=1&namedetails=1

        sqlx::query(
            r#"
            INSERT INTO
                stations
            (
                id,
                name,
                uic,
                latitude,
                longitude,
                country
            )
            VALUES (
                ?, ?, ?, ?, ?, ?
            )
            "#,
        )
        .bind(station.id)
        .bind(station.name)
        .bind(station.uic)
        .bind(station.latitude)
        .bind(station.longitude)
        .bind(station.country)
        .execute(&mut conn)
        .await?;
        i += 1;
    }

    Ok(i)
}

fn get_db_config(config: &DatabaseConfig) -> SqliteConnectOptions {
    let db_url = Url::parse(&config.url).expect("Invalid DATABASE_URL!");
    ConnectOptions::from_url(&db_url).expect("Invalid DATABASE_URL!")
}

async fn get_db_client(config: &DatabaseConfig) -> SqliteConnection {
    let db_config = get_db_config(config);
    let connection = SqliteConnection::connect_with(&db_config).await.unwrap();

    connection
}

fn get_cargo_path() -> Result<String, anyhow::Error> {
    std::env::var("CARGO")
        .map_err(|_| anyhow!("Please invoke me using Cargo, e.g.: `cargo db <ARGS>`"))
}

fn prepare_station(record: StringRecord, i: i32) -> Result<StationRecord, anyhow::Error> {
    let id = record.get(0);
    let name = record.get(1);
    let uic = record.get(3);
    let lat = record.get(5);
    let lon = record.get(6);
    let country = record.get(8);

    match (
        id, name, uic, lat, lon, country
    ) {
        (
            Some(id),
            Some(name),
            Some(uic),
            Some(lat),
            Some(lon),
            Some(country),
        ) => {
            let id = id
                .parse::<i64>()
                .context(format!("Failed to parse ID to i64: {}", id))?;
            let lat = if lat.trim().is_empty() {
                None
            } else {
                Some(
                    lat.parse::<f64>()
                        .context(format!("Failed to parse latitude to f64: {}", lat))?,
                )
            };
            let lon = if lon.trim().is_empty() {
                None
            } else {
                Some(
                    lon.parse::<f64>()
                        .context(format!("Failed to parse longitude to f64: {}", lon))?,
                )
            };

            Ok(StationRecord {
                id,
                name: name.to_string(),
                uic: uic.to_string(),
                latitude: lat,
                longitude: lon,
                country: prepare_csv_string(country),
            })
        }
        _ => Err(anyhow!("Invalid data in line {}!", i)),
    }
}

fn prepare_csv_string(input: &str) -> Option<String> {
    if input.trim().is_empty() {
        None
    } else {
        Some(String::from(input))
    }
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

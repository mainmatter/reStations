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
    pub info_de: Option<String>,
    pub info_en: Option<String>,
    pub info_es: Option<String>,
    pub info_fr: Option<String>,
    pub info_it: Option<String>,
    pub info_nb: Option<String>,
    pub info_nl: Option<String>,
    pub info_cs: Option<String>,
    pub info_da: Option<String>,
    pub info_hu: Option<String>,
    pub info_ja: Option<String>,
    pub info_ko: Option<String>,
    pub info_pl: Option<String>,
    pub info_pt: Option<String>,
    pub info_ru: Option<String>,
    pub info_sv: Option<String>,
    pub info_tr: Option<String>,
    pub info_zh: Option<String>,
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
                country,
                info_de,
                info_en,
                info_es,
                info_fr,
                info_it,
                info_nb,
                info_nl,
                info_cs,
                info_da,
                info_hu,
                info_ja,
                info_ko,
                info_pl,
                info_pt,
                info_ru,
                info_sv,
                info_tr,
                info_zh
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
        )
        .bind(station.id)
        .bind(station.name)
        .bind(station.uic)
        .bind(station.latitude)
        .bind(station.longitude)
        .bind(station.country)
        .bind(station.info_de)
        .bind(station.info_en)
        .bind(station.info_es)
        .bind(station.info_fr)
        .bind(station.info_it)
        .bind(station.info_nb)
        .bind(station.info_nl)
        .bind(station.info_cs)
        .bind(station.info_da)
        .bind(station.info_hu)
        .bind(station.info_ja)
        .bind(station.info_ko)
        .bind(station.info_pl)
        .bind(station.info_pt)
        .bind(station.info_ru)
        .bind(station.info_sv)
        .bind(station.info_tr)
        .bind(station.info_zh)
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
    let info_de = record.get(54);
    let info_en = record.get(55);
    let info_es = record.get(56);
    let info_fr = record.get(57);
    let info_it = record.get(58);
    let info_nb = record.get(59);
    let info_nl = record.get(60);
    let info_cs = record.get(61);
    let info_da = record.get(62);
    let info_hu = record.get(63);
    let info_ja = record.get(64);
    let info_ko = record.get(65);
    let info_pl = record.get(66);
    let info_pt = record.get(67);
    let info_ru = record.get(68);
    let info_sv = record.get(69);
    let info_tr = record.get(70);
    let info_zh = record.get(71);

    match (
        id, name, uic, lat, lon, country, info_de, info_en, info_es, info_fr, info_it, info_nb,
        info_nl, info_cs, info_da, info_hu, info_ja, info_ko, info_pl, info_pt, info_ru, info_sv,
        info_tr, info_zh,
    ) {
        (
            Some(id),
            Some(name),
            Some(uic),
            Some(lat),
            Some(lon),
            Some(country),
            Some(info_de),
            Some(info_en),
            Some(info_es),
            Some(info_fr),
            Some(info_it),
            Some(info_nb),
            Some(info_nl),
            Some(info_cs),
            Some(info_da),
            Some(info_hu),
            Some(info_ja),
            Some(info_ko),
            Some(info_pl),
            Some(info_pt),
            Some(info_ru),
            Some(info_sv),
            Some(info_tr),
            Some(info_zh),
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
                info_de: prepare_csv_string(info_de),
                info_en: prepare_csv_string(info_en),
                info_es: prepare_csv_string(info_es),
                info_fr: prepare_csv_string(info_fr),
                info_it: prepare_csv_string(info_it),
                info_nb: prepare_csv_string(info_nb),
                info_nl: prepare_csv_string(info_nl),
                info_cs: prepare_csv_string(info_cs),
                info_da: prepare_csv_string(info_da),
                info_hu: prepare_csv_string(info_hu),
                info_ja: prepare_csv_string(info_ja),
                info_ko: prepare_csv_string(info_ko),
                info_pl: prepare_csv_string(info_pl),
                info_pt: prepare_csv_string(info_pt),
                info_ru: prepare_csv_string(info_ru),
                info_sv: prepare_csv_string(info_sv),
                info_tr: prepare_csv_string(info_tr),
                info_zh: prepare_csv_string(info_zh),
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

use std::sync::Arc;

use online_test::domain::application::Core;
use online_test::inbound::server::Server;
use online_test::repository::connection::{AsyncSqliteConnectionManager, AsyncSqlitePool};
use online_test::repository::question::QuestionSqliteRepository;
use online_test::repository::score::ScoreSqliteRepository;
use online_test::cli;
use snafu::{prelude::*, Whatever};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Option<AppCommand>,
}

#[derive(Subcommand)]
enum AppCommand {
    /// Operate on database via CLI
    Cli {
        #[clap(subcommand)]
        command: CliCommand,
    },
    /// Run web server
    Serve,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Add a question to database
    Add,
    /// Remove a question from database
    Remove,
    /// List the last 100 questions
    List,
}

#[tokio::main(flavor = "current_thread")]
#[snafu::report]
async fn main() -> Result<(), Whatever> {
    let args = Args::parse();

    dotenvy::dotenv().whatever_context("Could not read server configurations for .env")?;
    let database_url = std::env::var("DATABASE_URL").unwrap_or("production/data.db".to_owned());
    let listening_ip = std::env::var("LISTENING_IP").unwrap_or("0.0.0.0".to_owned());
    let listening_port = std::env::var("LISTENING_PORT").unwrap_or("8080".to_owned());

    let config = AsyncSqliteConnectionManager::new(&database_url);
    let database_pool = AsyncSqlitePool::builder(config)
        .build()
        .map(|pool| Arc::new(pool))
        .whatever_context("Could not initialize database pool")?;

    let question_repository = Arc::new(QuestionSqliteRepository::new(Arc::clone(&database_pool)));
    let score_repository = Arc::new(ScoreSqliteRepository::new(Arc::clone(&database_pool)));

    match args.command.unwrap_or(AppCommand::Serve) {
        AppCommand::Cli { command } => match command {
            CliCommand::Add => {
                cli::run_add(question_repository).await?;
            },
            CliCommand::Remove => {
                cli::run_remove(question_repository).await?;
            },
            CliCommand::List => {
                cli::run_list(question_repository).await?;
            },
        }
        AppCommand::Serve => {
            let core = Arc::new(Core::new(question_repository, score_repository));

            let listening_addr = format!("{listening_ip}:{listening_port}");
            Server::new(listening_addr.parse().unwrap(), core)
                .await
                .whatever_context("Could not initialize server")?
                .serve()
                .await
                .whatever_context("Server error occurred")?;
        },
    }

    Ok(())
}

use dotenvy::dotenv;
use std::env;
use sqlx::sqlite::{SqlitePool};
use structopt::StructOpt;

pub(crate) mod database;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Add { password: String, description: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::Add { password, description}) => {
            database::add(&pool, &password, &description).await?;
            println!("「{}」 is added!", description);
        }
        None => println!("Set arguments."),
    }

    Ok(())
}

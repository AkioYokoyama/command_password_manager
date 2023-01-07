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
    List,
    Add { key: String, password: String },
    Delete { key: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::List) => {
            database::lists(&pool).await?;
        }
        Some(Command::Add { key, password }) => {
            database::add(&pool, &key, &password).await?;
            println!("「{}」 is added!", key);
        }
        Some(Command::Delete { key }) => {
            database::delete(&pool, &key).await?;
            println!("「{}」 is deleted!", key);
        }
        None => println!("Set arguments."),
    }

    Ok(())
}

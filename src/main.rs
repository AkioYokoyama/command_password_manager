use dotenvy::dotenv;
use std::env;
use sqlx::sqlite::{SqlitePool};
use structopt::StructOpt;

pub(crate) mod database;
pub(crate) mod user_interface;
pub(crate) mod clipboard;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    List,
    Add { key: String },
    Copy { key: String },
    Delete { key: String },
    Flush,
    Help,
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
        Some(Command::Add { key }) => {
            let password = user_interface::read_buffer(String::from("Enter the password."));
            database::add(&pool, &key, &password).await?;
            println!("「{}」 is added!", key);
        }
        Some(Command::Copy { key }) => {
            let password = database::find_by_key(&pool, &key).await?;
            clipboard::copy(password);
            println!("「{}」 is copied!", key);
        }
        Some(Command::Delete { key }) => {
            database::delete(&pool, &key).await?;
            println!("「{}」 is deleted!", key);
        }
        Some(Command::Flush) => {
            database::truncate(&pool).await?;
            println!("passwords deleted!");
        }
        Some(Command::Help) => {
            println!("-- list        : Display list of registered passwords.");
            println!("-- add <key>   : Register a password.");
            println!("-- copy <key>  : Copy the password specified by key to the clipboard.");
            println!("-- delete <key>: Delete a registered password.");
            println!("-- flush       : Remove all passwords.");
        }
        None => println!("Set arguments."),
    }

    Ok(())
}

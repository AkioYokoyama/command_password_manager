use dotenvy::dotenv;
use std::env;
use sqlx::sqlite::{SqlitePool};
use structopt::StructOpt;
use std::io;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub(crate) mod database;

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
            let password = read_buffer();
            database::add(&pool, &key, &password).await?;
            println!("「{}」 is added!", key);
        }
        Some(Command::Copy { key }) => {
            let password = database::find_by_key(&pool, &key).await?;
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(password.to_owned()).unwrap();
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
        None => println!("Set arguments."),
    }

    Ok(())
}

fn read_buffer() -> String {
    println!("> Enter the password.");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    return buffer.trim().to_string();
}

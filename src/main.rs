use dotenvy::dotenv;

fn main() {
    dotenv().expect(".env file not found");

    println!("Hello, world!");
}

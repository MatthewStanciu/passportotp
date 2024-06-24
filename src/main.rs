use clap::{Parser, Subcommand};

mod generate;
mod login;
mod util;

#[derive(Parser)]
#[command(name = "passportotp")]
#[command(about = "A CLI tool for enabling 2FA for your Purdue Hackers Passport", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Login,
    Generate,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Login) => login::login(),
        Some(Commands::Generate) => generate::generate(),
        None => {
            print!("passportotp\n\ngenerate: generates a new code\nlogin: logs into id\n")
        }
    }
}

use clap::Parser;
use crate::jwt::JwtAlgorithm;
use crate::util::verify_file;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    #[command(about = "Sign a token")]
    Sign(SignArgs),
    #[command(about = "Verify a token")]
    Verify(VerifyArgs),
}

#[derive(Parser, Debug)]
pub struct SignArgs {
    #[arg(long, value_parser = verify_file)]
    pub(crate) secret: String,
    #[arg(long)]
    pub(crate) algo: JwtAlgorithm,
    #[arg(long)]
    pub(crate) sub: String,
    #[arg(long)]
    pub(crate) aud: String,
    #[arg(long)]
    pub(crate) exp: String,
}

#[derive(Parser, Debug)]
pub struct VerifyArgs {
    #[arg(long, value_parser = verify_file)]
    pub(crate) secret: String,
    #[arg(short)]
    pub(crate) token: String,
    #[arg(long)]
    pub(crate) algo: JwtAlgorithm,
}

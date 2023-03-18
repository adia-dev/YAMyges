use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};


#[derive(Debug, Args)]
pub struct LoginArgs {
    /// 🌐 Your kordis (MyGES) username
    username: String,
    #[arg(long, short)]
    password: Option<String>,
}



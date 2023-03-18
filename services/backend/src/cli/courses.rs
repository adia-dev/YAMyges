use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

// TODO: WIP
#[derive(Debug, Args)]
pub struct CoursesArgs {
    /// 🚀 The start date for the mission! Format: YYYY-MM-DD
    #[arg(short, long)]
    name: Option<String>,

    /// 🏁 The end date for the mission! Format: YYYY-MM-DD
    #[arg(short, long)]
    end: Option<String>,

    /// 📅 The week number we're targeting!
    #[arg(short, long)]
    week: Option<u8>,
}

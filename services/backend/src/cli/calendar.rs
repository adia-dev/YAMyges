use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use once_cell::sync::OnceCell;

pub static INSTANCE: OnceCell<&'static str> = OnceCell::new();

#[derive(Debug, Args)]
pub struct CalendarArgs {
    /// 🚀 The start date for the mission! Format: YYYY-MM-DD
    #[arg(
        short,
        long,
        default_value = INSTANCE.get().unwrap()
    )]
    pub start: Option<String>,

    /// 🏁 The end date for the mission! Format: YYYY-MM-DD
    #[arg(
        short,
        long,
        default_value = INSTANCE.get().unwrap()
    )]
    pub end: Option<String>,

    /// 📅 The week number we're targeting!
    #[arg(short, long)]
    pub week: Option<u8>,

    /// ℹ Format for the parsed date
    #[arg(
        long,
        require_equals = true,
        value_name = "DATE_FMT",
        default_value = "%Y-%m-%d",
        default_missing_value = "YYYY-MM-DD"
    )]
    pub format: String,
}

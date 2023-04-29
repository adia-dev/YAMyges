use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

pub mod calendar;
pub mod courses;
pub mod login;

use calendar::CalendarArgs;
use courses::CoursesArgs;
use login::LoginArgs;

// TODO: maybe use the default attributes so that it gets filled using the Cargo.toml

#[derive(Parser, Debug)]
#[clap(color = concolor_clap::color_choice())]
#[command(name = "YAMyges CLI")]
#[command(author = "Abdoulaye Dia <adia14@myges.fr>")]
#[command(version = "1.0")]
#[command(
    about = "🔍 A CLI tool for accessing and manipulating data from MyGES.",
    long_about = "📚 YAMyges CLI is a command-line interface for interacting with MyGES.
With this tool, you can easily retrieve data on your courses, grades, schedule, and more, and perform various manipulations on the data. (WIP)

The tool provides options for specifying the data to retrieve, including:
    - start/end dates and week numbers.
    - output data in raw JSON format.
    - colored and pretty printed output.

Built with Rust and a few Clap .🚀"
)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// 🐍 Output response as a raw JSON string
    #[arg(long)]
    raw: bool,

    #[command(flatten)]
    color: concolor_clap::Color,

    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    ///// 🌐 Authenticate the user to MyGES and retrieve its token
    Login(LoginArgs),
    /// 🔑 Retrieves the authentication token if exists somewhere
    Token,
    /// 📅 commands related to the myges calendar
    Calendar(CalendarArgs),
    /// 📚 commands related to the myges calendar
    Courses(CoursesArgs),
}

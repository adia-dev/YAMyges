mod cli;
mod kordis;

use calendar::INSTANCE;
use cli::{calendar, Command};

use clap::Parser;

use chrono::{NaiveDate, NaiveDateTime, Utc};
use dotenv::dotenv;

use lazy_static::lazy_static;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[macro_use]
extern crate log;

fn parse_arg_date(arg: Option<String>, fmt: &str) -> Result<NaiveDateTime> {
    match arg {
        Some(s) => match NaiveDate::parse_from_str(&s, fmt) {
            Ok(date_time) => Ok(date_time.and_hms_opt(0, 0, 0).unwrap()),
            Err(error) => Err(error.into()),
        },
        _ => Err("The argument for this date is apparently empty, please fill up one date.".into()),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    lazy_static! {
        static ref NOW: String = Utc::now().date_naive().to_string();
    };

    INSTANCE.set(&NOW).unwrap();

    let cli_args = cli::Cli::parse();
    // println!("{:#?}", cli_args);

    match cli_args.command {
        Command::Token => {
            println!("Here is your kordis token: gneugneugneu")
        }
        Command::Calendar(calendar::CalendarArgs {
            start,
            end,
            week: _,
            format,
        }) => {
            // println!("Calendar: {:#?} {:#?} {:#?} {}", start, end, week, format);

            let start_date_time = parse_arg_date(start, &format);
            let end_date_time = parse_arg_date(end, &format);

            println!("start: {:#?}", start_date_time);
            println!("end: {:#?}", end_date_time);
        }
        _ => (),
    };

    env_logger::Builder::new()
        .filter_level(cli_args.verbose.log_level_filter())
        .init();

    info!("Debug Mode");

    Ok(())
}

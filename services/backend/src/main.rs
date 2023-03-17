#![allow(unused)]

mod cli;
mod kordis;

use clap::Parser;
use std::{env, thread::sleep};

use chrono::NaiveDateTime;
use dotenv::dotenv;
use kordis::KordisToken;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli_args = cli::Cli::parse();
    println!("{:#?}", cli_args);

    let args: Vec<String> = env::args().collect();

    env_logger::Builder::new()
        .filter_level(cli_args.verbose.log_level_filter())
        .init();

    info!("starting up");

    // let pb = indicatif::ProgressBar::new(100);
    //
    // for i in 0..3 {
    //     sleep(tokio::time::Duration::from_millis(i));
    //     pb.println(format!("[+] job #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");

    return;

    match args.get(1) {
        Some(arg_date) => {
            let arg_date_time: String = format!("{} 00:00:00", &arg_date);
            println!("{arg_date_time}");
            let parsed_date_time =
                NaiveDateTime::parse_from_str(&arg_date_time, "%Y-%m-%d %H:%M:%S").unwrap();

            let username: String =
                std::env::var("KORDIS_USERNAME").expect("KORDIS_USERNAME must be set.");

            let password: String =
                std::env::var("KORDIS_PASSWORD").expect("KORDIS_PASSWORD must be set.");

            let token: KordisToken = match kordis::authenticate(&username, &password).await {
                Ok(token) => token,
                Err(error) => panic!("Could not authenticate {}, reason: {:?}", username, error),
            };

            let start: i64 = parsed_date_time.timestamp_millis();
            let end: i64 = chrono::offset::Local::now()
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .timestamp_millis();

            println!("{:#?}", token);

            let agenda = token.get_agenda(start, end).await.unwrap();

            println!("{:#?}", agenda);
        }
        None => {
            println!("usage: {} YYYY-MM-DD", args.get(0).unwrap())
        }
    }
}

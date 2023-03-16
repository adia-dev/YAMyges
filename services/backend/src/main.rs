mod kordis;

use std::env;

use chrono::NaiveDateTime;
use dotenv::dotenv;
use kordis::KordisToken;

// TODO: add a CLI to allow the user to login using -u, --username, -p, --password
// --start YYYY-MM-DD
// --end YYYY-MM-DD
// -w +|- <number> to move in the calendar on a week basis
#[tokio::main]
async fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

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

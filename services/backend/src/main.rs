mod kordis;

use dotenv::dotenv;
use kordis::KordisToken;

// TODO: add a CLI to allow the user to login using -u, --username, -p, --password
#[tokio::main]
async fn main() {
    dotenv().ok();

    let username: String = std::env::var("KORDIS_USERNAME").expect("KORDIS_USERNAME must be set.");

    let password: String = std::env::var("KORDIS_PASSWORD").expect("KORDIS_PASSWORD must be set.");

    let token: KordisToken = match kordis::authenticate(&username, &password).await {
        Ok(token) => token,
        Err(error) => panic!("Could not authenticate {}, reason: {:?}", username, error),
    };

    println!("{:#?}", token);

    let agenda = token.get_agenda()
        .await
        .unwrap();

    println!("{:#?}", agenda);

}

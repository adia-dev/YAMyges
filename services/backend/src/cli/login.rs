use clap::Args;

#[derive(Debug, Args)]
pub struct LoginArgs {
    /// 🌐 Your kordis (MyGES) username
    username: String,
    #[arg(long, short)]
    password: Option<String>,
}

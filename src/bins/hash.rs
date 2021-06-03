use broker::config::Config;
use std::env;

fn main() {
    dotenv::dotenv().ok();
    let config = Config::from_env().unwrap();
    let auth = config.auth;
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(pass) => print!("{}", auth.hash_password(&pass)),
        None => panic!("Password is required"),
    }
}

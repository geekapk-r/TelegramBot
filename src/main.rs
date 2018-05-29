#[macro_use]
extern crate lazy_static;
extern crate teleborg;

use std::env::var;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;
use teleborg::objects::Update;
use teleborg::ParseMode::Markdown;
use teleborg::{Bot, Dispatcher, Updater};

static VERSION: &str = "0.0.8";
static API_VAR_NAME: &str = "TG_TOKEN";

lazy_static! {
    #[derive(Debug)]
    static ref APIKEY: String = {
        if let Ok(v) = var(&API_VAR_NAME) {v} else {
            println!("Set bot token to {} first.", API_VAR_NAME);
            ::std::process::exit(1);
        }
    };
}

fn main() {
    println!(
        "GeekApk bot is starting at unix time:{:?} with KEY:{}",
        SystemTime::now(),
        APIKEY.to_string()
    );
    let bot_token = APIKEY.to_string();
    let mut dispatcher = Dispatcher::new();
    dispatcher.add_command_handler("help", help, false);
    Updater::start(Some(bot_token), None, None, None, dispatcher);
}

fn help(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
    // Do nothing
}

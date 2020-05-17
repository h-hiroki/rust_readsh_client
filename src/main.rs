#[macro_use(dotenv)]
extern crate dotenv_codegen;

mod app_config;

use crate::app_config::get_config;

fn main() {
    let cnf = get_config();
    println!("{}", cnf.base_url);
    println!("{}", cnf.api_key);
}

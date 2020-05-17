#[macro_use(dotenv)]
extern crate dotenv_codegen;

mod app_config;

use crate::app_config::get_config;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cnf = get_config();

    // とりあえずAPI叩いてみる
    let url = format!("{}{}{}", cnf.base_url, "/api/queries/2168?api_key=", cnf.api_key);
    let resp = reqwest::get(&url).await?;
    println!("Status: {}", resp.status());
    let body = resp.text().await?;
    println!("Body:\n\n{}", body);
    Ok(())
}

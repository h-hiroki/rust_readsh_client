#[macro_use]
extern crate dotenv_codegen;

fn main() {
    println!("{}", dotenv!("BASE_URL"));
    println!("{}", dotenv!("API_KEY"));
}

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
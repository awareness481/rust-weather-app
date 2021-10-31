use error_chain::error_chain;
use std::env;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key: String = get_api_key();
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q=athens&appid={}", api_key);

    let res = reqwest::get(url).await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}

fn get_api_key() -> String {
    let u = match env::var_os("API_KEY") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$API_KEY is not set")
    };
    u
}
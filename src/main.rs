use error_chain::error_chain;
use serde_json::value::Value;
use std::env;
mod json;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key: String = get_api_key();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q=athens&appid={}",
        api_key
    );

    let res = reqwest::get(url).await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let j = json::to_json(body.clone());
    let weather = get_weather(&j);
    print_weather(weather);
    Ok(())
}

fn get_api_key() -> String {
    let u = match env::var_os("API_KEY") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$API_KEY is not set"),
    };
    u
}

struct W {
    feels_like: f64,
    temp: f64,
    temp_max: f64,
    temp_min: f64,
}

fn get_weather(v: &Value) -> W {
    let weather = v["main"].to_owned();
    let current_weather = W {
        feels_like: get_c_to_kelvin(weather["feels_like"].to_owned()),
        temp: get_c_to_kelvin(weather["temp"].to_owned()),
        temp_max: get_c_to_kelvin(weather["temp_max"].to_owned()),
        temp_min: get_c_to_kelvin(weather["temp_min"].to_owned()),
    };
    println!("{}", weather);
    current_weather
}

fn get_c_to_kelvin(k: Value) -> f64 {
    k.as_f64().unwrap() - 273.15
}

fn print_weather(weather: W) {
    println!(
        "The current temperature is {}°C and feels like {}°C \nMax and min temperatures are {}°C and {}°C",
        weather.temp.round(), weather.feels_like.round(), weather.temp_max.round(), weather.temp_min.round()
    )
}

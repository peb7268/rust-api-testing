#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

/*
Docs: https://www.weatherapi.com/docs/
Sample Endpoint: https://api.weatherapi.com/v1/current.json?key=c0949a108e054143a12180833232212&q=80129
Command to run and build app in watch mode: cargo watch -c -x run
key=c0949a108e054143a12180833232212
api_base http://api.weatherapi.com/v1
*/

use reqwest::Error;
use std::collections::HashMap;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct WeatherData {
    current: Value,
    location: Value
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let API_BASE: String = "http://api.weatherapi.com/v1".to_string();
    let TARGET_ENDPOINT: String = "/current.json".to_string();
    let KEY: String = "c0949a108e054143a12180833232212".to_string();
    let Q: String = "80129".to_string();

    let endpoint = format!("{}{}", API_BASE, TARGET_ENDPOINT);
    println!("Endpoint is: {}", &endpoint);

    // Define the query parameters
    let mut params = HashMap::new();
    params.insert("key", KEY);
    params.insert("q", Q);

     // Send the GET request
     let resp: reqwest::Response = reqwest::Client::new()
    .get(&endpoint)
    .query(&params)
    .send()
    .await?;
    
    let body_json: WeatherData = resp.json::<WeatherData>().await?;
    println!("{:#?}", body_json);

    Ok(())
}

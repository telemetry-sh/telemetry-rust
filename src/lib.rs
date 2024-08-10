use reqwest::blocking::{ Client };
use serde_json::{ json, Value };
use std::error::Error;

pub struct Telemetry {
    api_key: Option<String>,
    base_url: String,
}

impl Telemetry {
    pub fn new() -> Self {
        Self {
            api_key: None,
            base_url: "https://api.telemetry.sh".to_string(),
        }
    }

    pub fn init(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }

    pub fn log(&self, table: &str, data: &Value) -> Result<Value, Box<dyn Error>> {
        if self.api_key.is_none() {
            return Err("API key is not initialized. Please call init() with your API key.".into());
        }

        let client = Client::new();
        let body = json!({
            "data": data,
            "table": table,
        });

        let response = client
            .post(&format!("{}/log", self.base_url))
            .header("Content-Type", "application/json")
            .header("Authorization", self.api_key.as_ref().unwrap())
            .json(&body)
            .send()?;

        let json_response: Value = response.json()?;
        Ok(json_response)
    }

    pub fn query(&self, query: &str) -> Result<Value, Box<dyn Error>> {
        if self.api_key.is_none() {
            return Err("API key is not initialized. Please call init() with your API key.".into());
        }

        let client = Client::new();
        let body =
            json!({
            "query": query,
            "realtime": true,
            "json": true,
        });

        let response = client
            .post(&format!("{}/query", self.base_url))
            .header("Content-Type", "application/json")
            .header("Authorization", self.api_key.as_ref().unwrap())
            .json(&body)
            .send()?;

        let json_response: Value = response.json()?;
        Ok(json_response)
    }
}

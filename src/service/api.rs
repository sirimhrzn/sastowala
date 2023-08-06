use serde_json::Value;

use crate::error::CustomError;
use crate::error::ErrorResponse;
use reqwest::header::HeaderMap;
pub async fn get_from_seller_api(url: String) -> Result<Value, CustomError> {
    let response = reqwest::get(&url).await.map_err(|e| {
        CustomError::CustomMessage(ErrorResponse {
            message: e.to_string(),
        })
    })?;
    let data = response.text().await.map_err(|e| {
        CustomError::CustomMessage(ErrorResponse {
            message: e.to_string(),
        })
    })?;
    let json_data = serde_json::from_str(&data).map_err(|e| {
        CustomError::CustomMessage(ErrorResponse {
            message: e.to_string(),
        })
    })?;
    Ok(json_data)
}
pub async fn post_to_seller_api(url: &String, payload: &Value,headers:HeaderMap) -> Result<Value, CustomError> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(payload)
        .headers(headers)
        .send()
        .await
        .map_err(|e| {
            CustomError::CustomMessage(crate::error::ErrorResponse {
                message: e.to_string(),
            })
        })?;

    let res: Value = response.json::<Value>().await.map_err(|e| {
        CustomError::CustomMessage(ErrorResponse {
            message: e.to_string(),
        })
    })?;
    Ok(res)
}

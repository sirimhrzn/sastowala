use serde_json::Value;

use crate::error::CustomError;
use crate::error::ErrorResponse;

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

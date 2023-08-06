use std::env;

use crate::error::{CustomError, ErrorResponse};
use axum::Json;
use reqwest::header::HeaderValue;
use serde_json::{json, Value};

use super::api::post_to_seller_api;
pub struct HamroBazaarActions {
    pub query: String,
    pub category: String,
}
impl HamroBazaarActions {
    pub async fn get_by_query(
        payload: &Value,
    ) -> Result<Json<Vec<serde_json::Value>>, crate::error::CustomError> {
        let url = format!("https://api.hamrobazaar.com/api/Search/Products");
        let mut headers = reqwest::header::HeaderMap::new();
        let api_key = env::var("API_KEY").expect("API_KEY value invalid or not set");
        headers.insert(
            "Apikey",
            HeaderValue::from_str(&api_key).map_err(|e| {
                CustomError::CustomMessage(ErrorResponse {
                    message: e.to_string(),
                })
            })?,
        );
        let res_json = post_to_seller_api(&url, payload, headers).await?;
        let arr_json = res_json.get("data");
        let arr_exists = match arr_json {
            Some(data) => data.clone(),
            None => json!("no data"),
        };
        let this_arr = arr_exists.as_array();
        let arr_val = match this_arr {
            Some(arr) => arr,
            None => return Err(CustomError::FileReadFailed),
        };
        let mut res_vec: Vec<Value> = Vec::new();
        for obj in arr_val {
            let location = obj.get("location");
            let location_string = match location {
                Some(d) => d.clone(),
                None => json!({"location":"none"}),
            };
            let actual_location = location_string.get("locationDescription");
            let location_fr = match actual_location {
                Some(e) => {
                    let e_string = e.clone().to_string();
                    let words: Vec<&str> = e_string.split_whitespace().collect();
                    if let Some(last_word) = words.last() {
                        last_word.to_string()
                    } else {
                        "".to_string()
                    }
                }
                None => "".to_string(),
            };
            let mut url_l = "N/a".to_string();
            let name = match obj.get("name") {
                Some(name) => name.to_string(),
                None => "".to_string(),
            };
            let cat_name = match obj.get("categoryName") {
                Some(name) => name.clone(),
                None => json!(""),
            };
            let id = match obj.get("id") {
                Some(name) => name.to_string(),
                None => "".to_string(),
            };
            if location_fr != "".to_string() {
                url_l = location_fr;
                let custom_url = format!(
                    "https://hamrobazaar.com/{}/{}-in-{}/{}",
                    cat_name, name, url_l, id
                );
                url_l = custom_url.replace("\"", "");
            }
            let res_data = json!({
                "name": obj.get("name"),
                "image_url": obj.get("imageUrl"),
                "current_price": obj.get("price"),
                "vendor": "hamrobazaar",
                "free_shipping": "false",
                "rating_score":"N/A",
                "original_price": obj.get("price"),
                "product_url": url_l
            });
            res_vec.push(res_data.clone())
        }
        Ok(Json(res_vec))
    }
}

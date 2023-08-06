use crate::error::CustomError;
use axum::Json;
use serde_json::{json, Value};

use super::api::get_from_seller_api;
pub struct DarazActions {
    pub query: String,
    pub category: String,
}
impl DarazActions {
    pub async fn get_by_query(
        query: &String,
    ) -> Result<Json<Vec<serde_json::Value>>, crate::error::CustomError> {
        let url = format!(
            "https://www.daraz.com.np/catalog/?ajax=true&from=filter&q={}&rating=4",
            query
        );
        let res_json = get_from_seller_api(url).await?;
        let mods = res_json.get("mods");
        let mods_nest = match mods {
            Some(data) => data.clone(),
            None => return Err(CustomError::JsonParseFail),
        };
        let list_items = mods_nest.get("listItems");
        let list_items_nest = match list_items {
            Some(value) => value.clone(),
            None => json!("No items found"),
        };
        let ar_items = list_items_nest.as_array();
        let ar_items_exists = match ar_items {
            Some(data) => data.clone(),
            None => vec![json!("No Value")],
        };

        let mut arr: Vec<Value> = Vec::new();
        for item in ar_items_exists.iter() {
            let maplog = item.get("utLogMap");
            let maplog_exists = match maplog {
                Some(data) => data.clone(),
                None => json!("No additional data"),
            };
            let resp = json!({
                "name": item.get("name"),
                "current_price": item.get("price"),
                "product_url": item.get("productUrl"),
                "image_url": item.get("image"),
                "rating_score": item.get("ratingScore"),
                "free_shipping": maplog_exists.get("isFreeShipping"),
                "original_price": maplog_exists.get("originalPrice")
            });
            arr.push(resp)
        }
        return Ok(Json(arr));
    }
}

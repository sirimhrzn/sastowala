use crate::{database::database::DBstate, error::CustomError};
use axum::extract::State;
use axum::Json;
use reqwest;
use serde::Deserialize;
use serde_json::{json, Value};
#[derive(Deserialize)]
pub struct Login {
    pub lang: Option<String>,
    pub update: Option<String>,
}
pub async fn darazv1(_state: State<DBstate>) -> Result<Json<Vec<serde_json::Value>>, CustomError> {
    let response = reqwest::get(
        "https://www.daraz.com.np/catalog/?ajax=true&from=filter&q=type%20c%20charger&rating=4",
    )
    .await
    .unwrap();
    let text = response.text().await.unwrap();
    let res_json: Value = serde_json::from_str(&text).unwrap();
    let mods = res_json.get("mods");
    let mods_nest = match mods {
        Some(data) => data.clone(),
        None => json!("No data"),
    };
    let list_items = mods_nest.get("listItems");
    let list_items_nest = match list_items {
        Some(value) => value.clone(),
        None => json!("No items found"),
    };
    let ar_items = list_items_nest.as_array();
    let ar_items_exists = match ar_items {
        Some(data) => data.clone(),
        None => vec![json!("no value")],
    };

    //   dbg!(&list_items_nest);
    let mut arr: Vec<Value> = Vec::new();
    for item in ar_items_exists.iter() {
        let maplog = item.get("utLogMap");
        let maplog_exists = match maplog {
            Some(data) => data.clone(),
            None => json!("no data"),
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

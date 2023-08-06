use crate::service::hamrobazar::HamroBazaarActions;
use crate::{database::database::DBstate, error::CustomError, service::daraz::DarazActions};
use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;
#[derive(Deserialize)]
pub struct QueryParam {
    pub search: Option<String>,
}
pub async fn darazv1(
    _state: State<DBstate>,
    Query(query): Query<QueryParam>,
) -> Result<Json<Vec<serde_json::Value>>, CustomError> {
    let params = match query.search {
        Some(q) => q,
        None => "latest".to_string(),
    };

    let data = DarazActions::get_by_query(&params).await?;
    Ok(data)
}

pub async fn hamro_bazarv1(
    Query(query): Query<QueryParam>,
) -> Result<Json<Vec<serde_json::Value>>, CustomError> {
    let params = match query.search {
        Some(q) => q,
        None => "latest".to_string(),
    };
    let payload = json!(
    {"pageNumber":1,"pageSize":27,
    "latitude":0,"longitude":0,
    "deviceId":"a0ddb0ee-282f-4d2d-ad2f-2103089s92a5",
    "deviceSource":"web","isHBSelect":false,
    "searchParams":{"searchValue":params,"searchBy":"",
    "searchByDistance":0},"filterParams":
    {"condition":0,"priceFrom":0,"priceTo":0,
    "isPriceNegotiable":null,"category":"","brand":"",
    "categoryIds":"","brandIds":"","advanceFilter":""},
    "sortParam":0});
    let response = HamroBazaarActions::get_by_query(&payload).await?;
    Ok(response)

}

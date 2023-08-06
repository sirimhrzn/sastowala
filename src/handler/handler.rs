use crate::{database::database::DBstate, error::CustomError, service::daraz::DarazActions};
use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct QueryParam {
    pub search: Option<String>,
}
pub async fn darazv1(
    _state: State<DBstate>,
    Query(query): Query<QueryParam>,
) -> Result<Json<Vec<serde_json::Value>>, CustomError> {
    let params = match query.search {
        Some(quer) => quer,
        None => "latest".to_string(),
    };

    let data = DarazActions::get_by_query(&params).await?;
    Ok(data)
}

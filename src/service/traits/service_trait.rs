use serde_json::Value;

use crate::error::CustomError;

pub trait Services {
    fn new() -> Self;
    fn get_by_query(query: String) -> Result<Value, CustomError>;
    fn get_most_selling() -> Result<Value, CustomError>;
}

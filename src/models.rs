use serde::{Deserialize, Serialize};

/// Represents Pizza Business
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Business {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String
}
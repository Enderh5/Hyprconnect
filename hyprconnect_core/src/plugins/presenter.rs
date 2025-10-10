use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Presenter {
    pub dx: Option<f64>,
    pub dy: Option<f64>,
    pub stop: Option<bool>,
}

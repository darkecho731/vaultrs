use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::sys::responses::MountResponse;

#[derive(Deserialize, Debug, Serialize)]
pub struct ListMountsResponse {
    pub auth: HashMap<String, MountResponse>,
    pub secret: HashMap<String, MountResponse>,
}

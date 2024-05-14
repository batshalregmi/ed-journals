use serde::{Serialize, Deserialize};

use crate::modules::models::ship::ship_module::ShipModule;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RepairEvent {
    pub items: Vec<String>,
    pub cost: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum RepairEventItem {
    Hull,
    Wear,
    Module(ShipModule),
}

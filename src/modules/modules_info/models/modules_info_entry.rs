use crate::modules::ship::{ShipModule, ShipSlot};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModulesInfoEntry {
    pub slot: ShipSlot,
    pub item: ShipModule,
    pub power: f32,
    pub priority: Option<u8>,
}

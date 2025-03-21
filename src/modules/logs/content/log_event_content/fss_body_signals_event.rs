use serde::{Deserialize, Serialize};

use crate::modules::exploration::PlanetarySignalType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSBodySignalsEvent {
    pub body_name: String,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub system_address: u64,
    pub signals: Vec<FSSBodySignalEventSignal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSBodySignalEventSignal {
    #[serde(rename = "Type")]
    pub kind: PlanetarySignalType,

    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,
    pub count: u8,
}

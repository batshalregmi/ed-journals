use std::fmt::{Display, Formatter};
use serde::Deserialize;

use crate::models::journal_event_content::shared::ship::ship_module::ship_cockpit_module::ShipCockpitModule;
use crate::models::journal_event_content::shared::ship::ship_module::ship_hardpoint_module::ShipHardpointModule;
use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::ShipInternalModule;

pub mod module_class;
pub mod ship_cockpit_module;
pub mod ship_hardpoint_module;
pub mod ship_internal_module;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ShipModule {
    /// Special case for the cargo bay door.
    #[serde(rename = "$modularcargobaydoor_name;")]
    CargoBayDoor,

    /// Any internal module, this includes core and optional modules.
    #[serde(untagged)]
    Internal(ShipInternalModule),

    /// For external modules, both full-sized hardpoints and utility modules.
    #[serde(untagged)]
    Hardpoint(ShipHardpointModule),

    // #[serde(untagged)]
    // Armor(ShipArmorModule),
    #[serde(untagged)]
    Cockpit(ShipCockpitModule),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for ShipModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipModule::CargoBayDoor => write!(f, "Cargo Hatch"),
            ShipModule::Internal(internal_module) => internal_module.fmt(f),
            ShipModule::Hardpoint(hardpoint_module) => hardpoint_module.fmt(f),
            ShipModule::Cockpit(_) => write!(f, "Cockpit"),
            ShipModule::Unknown(unknown) => write!(f, "Unknown module: {}", unknown),
        }
    }
}

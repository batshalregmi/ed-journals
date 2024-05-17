use crate::modules::models::odyssey::suit::Suit;
use crate::modules::models::odyssey::suit_mod::SuitMod;
use crate::modules::models::odyssey::suit_slot::SuitSlot;
use crate::modules::models::odyssey::weapon::Weapon;
use crate::modules::models::odyssey::weapon_mod::WeaponMod;
use serde::{Serialize, Deserialize};
use crate::models::odyssey::suit_module::SuitModule;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SuitLoadoutEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_name: Suit,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localized: String,
    pub suit_mods: Vec<SuitMod>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: String,
    pub modules: Vec<SuitModule>,
}

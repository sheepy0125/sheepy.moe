//! Constants

use crate::types::Button88x31;
use lazy_static::lazy_static;

pub const GLOBAL_COUNTER_COOKIE_KEY: &str = "count";
pub const CONFIG_COOKIE_KEY: &str = "config";

lazy_static! {
    pub static ref BUTTON_88X31S: Vec<Button88x31> =
        ron::de::from_str(include_str!("../static/88x31/listing.ron")).unwrap();
}

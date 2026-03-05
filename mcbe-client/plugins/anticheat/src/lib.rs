use log::*;
use pumpkin_api_macros::plugin_impl;

#[plugin_impl]
pub struct Anticheat;

impl Default for Anticheat {
    fn default() -> Self {
        Self::new()
    }
}

impl Anticheat {
    pub fn new() -> Self {
        Self {}
    }
}

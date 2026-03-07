use pumpkin_api_macros::plugin_impl;

#[plugin_impl]
pub struct AnticheatPlugin {}

impl AnticheatPlugin {
    pub fn new() -> Self {
        AnticheatPlugin {}
    }
}

impl Default for AnticheatPlugin {
    fn default() -> Self {
        Self::new()
    }
}

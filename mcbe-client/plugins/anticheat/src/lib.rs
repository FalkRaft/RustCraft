#![allow(clippy::async_yields_async)]
use pumpkin_api_macros::{plugin_impl, plugin_method};

#[plugin_method]
async fn on_load(
    &mut self,
    server: std::sync::Arc<pumpkin::plugin::Context>,
) -> Result<(), String> {
    server.init_log();
    log::info!("[Anticheat] Loaded Plugin!");
    Ok(())
}

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

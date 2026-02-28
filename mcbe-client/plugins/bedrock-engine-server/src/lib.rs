use log::error;
use pumpkin::plugin::Context;
use pumpkin_api_macros::{plugin_impl, plugin_method};
use quickjs_rs::Context as JsContext;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

// ===== JS thread + command API =====

pub enum JsCommand {
    Eval(String),
    // You can add more commands here (e.g. CallFunction, LoadFile, etc.)
}

pub struct JsThread {
    sender: Sender<JsCommand>,
}

impl JsThread {
    #[must_use]
    pub fn start() -> Self {
        let (tx, rx): (Sender<JsCommand>, Receiver<JsCommand>) = channel();

        thread::spawn(move || {
            let js_ctx = JsContext::builder()
                .memory_limit(4_194_304)
                .build()
                .expect("Failed to create QuickJS context");

            // Single-threaded JS event loop
            while let Ok(cmd) = rx.recv() {
                match cmd {
                    JsCommand::Eval(code) => {
                        if let Err(err) = js_ctx.eval(code.as_str()) {
                            error!("QuickJS eval error: {err:?}");
                        }
                    }
                }
            }
        });

        Self { sender: tx }
    }

    pub fn eval<S: Into<String>>(&self, code: S) {
        // Fire-and-forget; you can extend this to return results if needed
        let _ = self.sender.send(JsCommand::Eval(code.into()));
    }
}

// ===== Pumpkin plugin definition =====

#[plugin_impl]
pub struct BedrockEnginePlugin {
    js: JsThread,
}

impl BedrockEnginePlugin {
    #[must_use]
    pub fn new() -> Self {
        Self {
            js: JsThread::start(),
        }
    }
}

impl Default for BedrockEnginePlugin {
    fn default() -> Self {
        Self::new()
    }
}

// ===== Plugin lifecycle methods =====

#[plugin_method]
async fn on_load(server: &Arc<Context>, bedrock_engine_plugin: &BedrockEnginePlugin) -> Result<(), String> {
    info!("Bedrock Engine plugin loaded!");

    // Example: run some JS on startup
    self.js.eval(r#"
        globalThis.hello = "Hello from QuickJS!";
        console.log(hello);
    "#);

    // You can also interact with `server` here if needed
    let _ = server; // just to silence unused warning for now
    server.log("Bedrock Engine plugin has access to the server context!");

    bedrock_engine_plugin.js.eval(r#"
        console.log("Bedrock Engine plugin is running!");
    "#);

    bedrock_engine_plugin.js.sender.send(JsCommand::Eval("Bedrock Engine plugin running!".to_string()));

    Ok(())
}

use log::{Level, LevelFilter, Metadata, Record};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

static LOGGER: TauriLogger = TauriLogger;

struct TauriLogger;

#[derive(Serialize)]
struct LogArgs {
    level: u16,
    message: String,
    location: Option<String>,
    file: Option<String>,
    line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_values: Option<String>,
}

impl log::Log for TauriLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let level = match record.level() {
            Level::Trace => 1,
            Level::Debug => 2,
            Level::Info => 3,
            Level::Warn => 4,
            Level::Error => 5,
        };
        let args = LogArgs {
            level,
            message: record.args().to_string(),
            location: None,
            file: record.file().map(str::to_string),
            line: record.line(),
            key_values: None,
        };
        let args = serde_wasm_bindgen::to_value(&args).expect("failed to serialize log args");
        wasm_bindgen_futures::spawn_local(async move {
            let _ = invoke("plugin:log|log", args).await;
        });
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&LOGGER).expect("logger already set");
    log::set_max_level(LevelFilter::Trace);
}

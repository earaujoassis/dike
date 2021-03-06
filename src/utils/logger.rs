use std::io::stdout;
use std::env;
use std::sync::Mutex;

use slog::*;
use slog_term;
use slog_json::Json;

pub fn logger_factory() -> Logger {
    let log_output_type = env::var("DIKE_LOG_OUTPUT").expect("DIKE_LOG_OUTPUT must be set");

    match log_output_type.as_ref() {
        "json" => {
            let json = Mutex::new(Json::default(stdout())).map(Fuse);

            Logger::root(
                json,
                o!("app" => "dike")
            )
        },
        _ => {
            let plain = slog_term::PlainSyncDecorator::new(stdout());
            let drain = slog_term::FullFormat::new(plain).build().fuse();

            Logger::root(
                drain,
                o!("app" => "dike")
            )
        }
    }
}

use std::str::FromStr;
use log::*;
use tracing_subscriber::{filter::{Targets, filter_fn}, prelude::*};

mod inner {
    use super::*;

    #[tracing::instrument(fields(yak))]
    pub fn log_smt(yak: u32) {
        debug!("inner - yak: {} - this is debug", yak);
        info!("inner - yak: {} - this is info", yak);
        warn!("inner - yak: {} - this is warn", yak);
    }
}

fn main() -> Result<(), std::io::Error> {

    let filter = Targets::from_str("debug,tracing_test::inner=warn").unwrap();

    let fmt_layer = tracing_subscriber::fmt::layer()
    .with_filter(filter_fn(move |_meta| true ));

    tracing_subscriber::registry().with(filter).with(fmt_layer).init();

    debug!("before inner");
    inner::log_smt(11111);
    debug!("after inner");

    Ok(())
}
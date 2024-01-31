use chrono::{DateTime, Utc};

fn main() {
    "2015-02-18T23:16:9.15øøø".parse::<DateTime<Utc>>();
}

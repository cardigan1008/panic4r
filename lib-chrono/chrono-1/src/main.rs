use chrono::{TimeZone, Utc};

fn main() {
    let nanos = i64::MIN + 2;
    let dt = Utc.timestamp_nanos(nanos);
    let nanos2 = dt.timestamp_nanos();
}

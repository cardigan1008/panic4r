use chrono::{Duration, DurationRound, TimeZone, Utc};

fn main() {
    let dt = Utc.ymd(2016, 12, 31).and_hms_nano(23, 59, 59, 175_500_000);
    dt.duration_round(Duration::zero()).unwrap().to_string();
}

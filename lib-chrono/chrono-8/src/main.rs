fn main() {
    let _local0 = chrono::naive::NaiveDateTime::from_timestamp_opt(320041586, 1920103021);
    let _local1 = chrono::Duration::nanoseconds(-8923838508697114584);
    let _local2_param0_helper1 = _local0.unwrap();
    chrono::DurationRound::duration_round(_local2_param0_helper1, _local1);
}

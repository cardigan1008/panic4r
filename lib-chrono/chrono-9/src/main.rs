fn main() {
    let _local0 = chrono::naive::NaiveDateTime::from_timestamp_opt(-2621440, 0);
    let _local1 = chrono::Duration::nanoseconds(-9223372036854771421);
    let _local2_param0_helper1 = _local0.unwrap();
    chrono::DurationRound::duration_round(_local2_param0_helper1, _local1);
}

fn main() {
    let _local0 = chrono::naive::NaiveDateTime::from_timestamp_opt(-4227854320, 1678774288);
    let _local1 = chrono::Duration::microseconds(-7019067213869040);
    let _local2_param0_helper1 = _local0.unwrap();
    chrono::DurationRound::duration_trunc(_local2_param0_helper1, _local1);
}

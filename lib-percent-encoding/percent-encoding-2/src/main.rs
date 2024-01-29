fn main() {
    let mut _local0 = url::Url::parse("o://").unwrap();
    let mut _local1 = url::Url::path_segments_mut(&mut _local0).unwrap();
    let _ = url::PathSegmentsMut::pop(&mut _local1);
}

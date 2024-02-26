use der::asn1::UtcTime;
use to_vec::ToVec;
fn main() {
    let test_time = UtcTime::new(std::time::Duration::from_secs(946684800u64)).unwrap();
    let der_encoded_time = test_time.to_vec().unwrap();
}
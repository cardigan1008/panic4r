use uuid;

fn main() {
    uuid::Timestamp::from_rfc4122(u64::MAX, 0); // ok
    uuid::Timestamp::from_rfc4122(122192928000000000, 0); // ok
    uuid::Timestamp::from_rfc4122(122192927999999999, 0); // panics
    uuid::Timestamp::from_rfc4122(0, 0); // panics
}
fn main() {
    let f: Result<f64, _> = serde_json::from_str("3.5E-2147483647");
}

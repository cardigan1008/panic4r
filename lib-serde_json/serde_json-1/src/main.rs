fn main() {
    let json_str2 = &[ b'"', b'\xCE', b'\xF8', b'"'];
    let v2: serde_json::Result<Box<serde_json::value::RawValue>> = serde_json::from_slice(json_str2);
}

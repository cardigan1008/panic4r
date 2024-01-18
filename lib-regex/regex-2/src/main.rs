fn main() {
    let needles = vec![
        "aA", "bA", "cA", "dA", "eA", "fA", "gA", "hA", "iA", "jA", "kA", "lA",
        "mA", "nA", "oA", "pA", "qA", "rA", "sA", "tA", "uA", "vA", "wA", "xA",
        "yA", "zA",
    ];
    let pattern = needles.join("|");
    let re = regex::Regex::new(&pattern).unwrap();
    let hay = "FUBAR";
    assert_eq!(0, re.find_iter(hay).count());
}
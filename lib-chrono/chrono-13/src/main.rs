use chrono::format;

fn main() {
    let p = &mut format::Parsed::new();
    format::parse(p, "MONYAŽA Šander`",
                  format::StrftimeItems::new("%A, %e-%h-%y %H:%M:%S"));

    println!("No panic");
}
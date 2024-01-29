fn main() {
    let date_ = time::Date::try_from_ymd(-36 ,11 ,1).unwrap();
    let _ = time::Date::weekday(date_);
}

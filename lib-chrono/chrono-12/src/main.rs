fn main() {
    let mut s = "1%Z%I%A%Z%I%A\u{7f}\u{1c} 4ThuP0\u{7f}\n\u{2000}\n\n\u{2000}\n\nJ \u{0} %Z%s%Z%\u{0}%s%Zsssssssssssssssssss%sZ%I\nJ \n3%Z%";
    let result = chrono::DateTime::parse_from_str(&s, &s);
}
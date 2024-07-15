use std::fs;
use filetime::FileTime;

fn main() {
    let metadata = fs::metadata("foo.txt").unwrap();
    let ctime = FileTime::from_creation_time(&metadata).unwrap();
    println!("{}", ctime);
}
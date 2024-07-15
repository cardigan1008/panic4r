use rand::{thread_rng, Rng};
fn main(){
let mut a: Vec<i32> = Vec::new();
thread_rng().fill(&mut a[..]);
}
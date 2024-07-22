use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, MultiProgress};

fn main() {
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(10));

    // start a thread to drive MultiProgress
    let h = thread::spawn(move || {
        m.join().unwrap();
        println!("Done in thread, droping MultiProgress");
    });

    {
        let pb2 = pb.clone();
        for _ in 0..10 {
            pb2.inc(1);
            thread::sleep(Duration::from_millis(50));
        }
    }

    h.join().unwrap();

    println!("Shouldn't have gone this far, as we haven't call finish yet");

    pb.set_message("Done");
    pb.finish();

    println!("Done in main");
}
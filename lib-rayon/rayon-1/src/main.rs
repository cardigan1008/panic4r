use rayon::prelude::*;
use tokio::runtime::Runtime;

fn main() {
    const NUM_THREADS: u64 = 1024;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS as usize)
        .build()
        .unwrap();
    pool.install(|| {
        (0..NUM_THREADS).into_par_iter().for_each(|_| {
        });
    });
    // I don't actually do anything with this runtime, but removing this line fixes the crash.
    let _rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
}
use parking_lot::ReentrantMutexGuard;
use parking_lot::ReentrantMutex;
use std::sync::Arc;

fn main() {
    let mutex = Arc::new(ReentrantMutex::new(()));
    let mut guard = mutex.lock();

    let mutex2 = Arc::clone(&mutex);

    std::thread::spawn(move || {
        let _guard = mutex2.lock();
    });

    // give the thread some time to try and lock
    std::thread::sleep(std::time::Duration::from_secs(2));

    ReentrantMutexGuard::bump(&mut guard);
}
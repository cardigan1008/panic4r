use std::{sync::Arc, time::Duration};

use crossbeam_skiplist::SkipMap;

fn main() {
    let map: Arc<SkipMap<u32, u32>> = Arc::new(SkipMap::new());
    map.insert(1, 2);
    let map1 = map.clone();
    std::thread::spawn(move||{
        let key = 1;
        for _ in 0..10_0000 {
            let len = map1.len();
            if let Some(entry) = map1.get(&key) {

            }else{
                panic!("len={},key={}",len,key);
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    });
    for _ in 0..10_0000 {
        map.insert(1, 2);
        std::thread::sleep(Duration::from_millis(100));
    }
}
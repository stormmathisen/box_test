use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct PV {
    pv_name: String,
    pv_value: i64
}

impl PV {
    fn new(name: String) -> PV {
        let mut tmp = PV {
            pv_name: name,
            pv_value: 0
        };
        tmp
    }

    fn get(&self) -> i64 {
        self.pv_value
    }
    fn put(&mut self, value: i64) -> bool {
        self.pv_value = value;
        return true
    }
}

fn main() {
    println!("Hello, world!");
    let wcm = Arc::new(Mutex::new(PV::new("CLA-S01-DIA-WCM-01:Q".to_string())));
    let data = Arc::clone(&wcm);
    let data_handle = thread::spawn(move || {
        for i in 0..256 {
            let start = Instant::now();
            println!("Data thread is putting {}", i);
            let mut wcm = data.lock().unwrap();
            println!("Waited for lock for {} us", start.elapsed().as_micros());
            wcm.put(i);
            drop(wcm);
            println!("Writing took {} us", start.elapsed().as_micros());
            thread::sleep(Duration::from_millis(100));
        }
    });

    let mut handles:Vec<thread::JoinHandle<()>> = Vec::new();
    for i in 0..10 {
        let data = Arc::clone(&wcm);
        let handle = thread::spawn(move || {
            for _ in 0..256 {
                let wcm = data.lock().unwrap();
                println!("Thread {} got {}", i, wcm.get());
                drop(wcm);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }
    data_handle.join().unwrap();
    for handle in handles {
        handle.join().unwrap();
    }
}

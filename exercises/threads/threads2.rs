use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
    // 添加一个互斥锁来保护jobs_completed的访问
    lock: Mutex<u32>,
}

impl JobStatus {
    fn new() -> Self {
        JobStatus {
            jobs_completed: 0,
            lock: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut guard = self.lock.lock().unwrap();
        *guard += 1;
    }
}

fn main() {
    let status = Arc::new(JobStatus::new());
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Update the shared value
            status_shared.increment();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // Print the value of JobStatus.jobs_completed after all threads have completed
    println!("jobs completed {}", status.lock.lock().unwrap().clone());
}

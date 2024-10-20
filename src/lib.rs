use std::sync::{Arc, Condvar, Mutex};

pub struct Semaphore {
    count: Mutex<isize>,
    cvar: Condvar,
}

impl Semaphore {
    pub fn new(count: isize) -> Self {
        Semaphore {
            count: Mutex::new(count),
            cvar: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.cvar.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_semaphore() {
        let sem = Arc::new(Semaphore::new(2));
        let mut handles = vec![];

        for _ in 0..5 {
            let sem_clone = Arc::clone(&sem);
            handles.push(thread::spawn(move || {
                sem_clone.acquire();
                println!("Thread acquired semaphore");
                thread::sleep(std::time::Duration::from_millis(100));
                sem_clone.release();
                println!("Thread released semaphore");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
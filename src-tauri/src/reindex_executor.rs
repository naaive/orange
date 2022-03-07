use std::thread;
use blockingqueue::BlockingQueue;

pub struct ReindexExecutor {
    paths: BlockingQueue<String>
}

impl ReindexExecutor {
    pub fn new() -> ReindexExecutor {
        let queue = BlockingQueue::new();
        let q = queue.clone();
        thread::spawn(move || {
            loop {
                let path = q.pop();

            }
        });
        ReindexExecutor {
            paths:queue
        }
    }
    pub fn submit(&mut self ,path: String){
        self.paths.push(path);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use blockingqueue::BlockingQueue;
    use std::{thread, time};

    #[test]
    fn t1() {
            let bq = BlockingQueue::new();
            let bq_clone1 = bq.clone();
            thread::spawn(move || {
                thread::sleep(time::Duration::from_millis(100));
                bq_clone1.push(123);
                bq_clone1.push(456);
                bq_clone1.push(789);
            });

            let bq_clone2 = bq.clone();
            thread::spawn(move || {
                thread::sleep(time::Duration::from_millis(400));
                bq_clone2.push(321);
                bq_clone2.push(654);
                bq_clone2.push(987);
            });

            let bq_clone3 = bq.clone();
            let read_three_thread = thread::spawn(move || {
                for _ in 0..3 {
                    println!("Popped in child thread: {}", bq_clone3.pop());
                }
            });

            for _ in 0..3 {
                println!("Popped in parent thread: {}", bq.pop());
            }

            read_three_thread.join().unwrap();
            println!("I will wait forever here...");
            println!("{}", bq.pop());

    }
}
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Queue>, tx: mpsc::Sender<u32>) -> () {
    let qc1 = Arc::clone(&q);
    let qc2 = Arc::clone(&q);

    let mut value1 = tx.send(qc1.first_half[0]).unwrap();
    for val in &qc1.first_half[1..] {
        println!("sending {:?}", val);
        value1 = tx.send(*val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    let mut value2 = tx.send(qc2.second_half[0]).unwrap();
    for val in &qc2.second_half[1..] {
        println!("sending {:?}", val);
        value2 = tx.send(*val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}


fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}

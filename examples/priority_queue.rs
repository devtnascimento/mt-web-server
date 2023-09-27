#[allow(dead_code)]

use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt::Debug;



// Define a custom struct to represent items with priorities.
#[derive(Debug, PartialEq, Eq, Clone)]
struct PriorityItem<T: 'static + PartialEq + Eq + Debug> {
    item: T,
    priority: i32,
}

impl<T: PartialEq + Eq + Debug> Ord for PriorityItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<T: PartialEq + Eq + Debug> PartialOrd for PriorityItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct RequestQueue<T: 'static + Eq + Debug + Send> {
    queue: Arc<Mutex<BinaryHeap<PriorityItem<T>>>>,
}

impl<T: Eq + Debug + Send> RequestQueue<T> {
    fn new() -> Self {
        RequestQueue {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
        }
    }

    fn push_back(&self, request: T, priority: i32) {
        self.queue.lock().unwrap().push(PriorityItem { item: request, priority });
    }

    fn set_priority(&self, request: T) -> i32 {
        println!("Resquest => {:?}", request);
        0
    }

    // fn pop(&self) -> Option<T> {
    //     self.queue.lock().unwrap().pop().map(|item| item.item)
    // }

    fn consume(&self) {

        println!("Consumindo a fila");
        let queue = self.queue.clone();
        let mut times: i16 = 0;

        thread::spawn(move || {
            loop {
                println!("Consumindo a fila ...");
                if let Some(item) = queue.lock().unwrap().pop() {
                    // Process the item here, e.g., print it.
                    println!("Consumed: {:?}", item);
                } else {
                    println!("Empty");
                    if times >= 3 {
                        break;
                    } 
                    thread::sleep(std::time::Duration::from_secs(1));
                    times += 1;
                }
            }
        });
    }
}


fn main() {
    // Create a RequestQueue.
    let request_queue = RequestQueue::new();

    // Start the consumer thread.
    request_queue.consume();

    // Enqueue items from the main thread.
    for i in 0..5 {
        request_queue.push_back(format!("Task {}", i), i);
    }


    // Sleep to allow time for the consumer thread to process items.
    //thread::sleep(std::time::Duration::from_secs(5));
}
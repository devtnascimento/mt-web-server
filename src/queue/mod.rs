use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt::Debug;



// Define a custom struct to represent items with priorities.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PriorityItem <T: 'static + PartialEq + Eq + Debug> {
    pub item: T,
    pub priority: i32,
}

impl <T: 'static + PartialEq + Eq + Debug> PriorityItem <T>{
    pub fn new(item: T, priority: i32) -> Self {
        Self {
            item,
            priority
        }
    }
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
pub struct RequestQueue<T: 'static + Eq + Debug + Send> {
    pub queue: Arc<Mutex<BinaryHeap<PriorityItem<T>>>>,
}

impl<T: Eq + Debug + Send> RequestQueue<T> {
    pub fn new() -> Self {
        RequestQueue {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
        }
    }

    pub fn push_back(&self, request: T, priority: i32) {
        self.queue.lock().unwrap().push(PriorityItem{item: request, priority});
    }

    pub fn pop(&self) -> Option<T> {
        self.queue.lock().unwrap().pop().map(|item| item.item)
    }

    pub fn consume(&self) {

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
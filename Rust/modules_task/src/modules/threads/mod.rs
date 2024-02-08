use rand::Rng;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Data {
    id: i32,
    name: String,
    timestamp: Instant,
}
pub fn threads() {
    let arr: Arc<RwLock<Vec<Data>>> = Arc::new(RwLock::new(Vec::new()));
    let ref1 = Arc::clone(&arr);
    let ref2 = Arc::clone(&arr);
    let ref3 = Arc::clone(&arr);

    let t1 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let length_ref = ref1.read();
        match length_ref {
            Ok(length_ref) => {
                println!("length of data vec is {}", length_ref.len());
            }
            Err(_) => {
                println!("thread panicked while reading length of vec");
            }
        }
    });

    let t2 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let write_ref = ref2.write();
        match write_ref {
            Ok(mut write_ref) => {
                let struct_data = rand_data();
                write_ref.push(Data {
                    id: struct_data.0,
                    name: struct_data.1,
                    timestamp: struct_data.2,
                });
                println!("data added successfully");
                // println!("{:#?}",write_ref);
            }
            Err(_) => {
                println!("thread panicked while writing data");
            }
        }
    });

    let t3 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(15));
        let deleting_ref = ref3.write();
        match deleting_ref {
            Ok(mut deleting_ref) => {
                let now = Instant::now();
                deleting_ref
                    .retain(|item| now.duration_since(item.timestamp) < Duration::from_secs(5));

                println!("removing data...");
            }
            Err(_) => {
                println!("thread panicked while removing data");
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}

fn rand_data() -> (i32, String, Instant) {
    let names = vec![
        "Jasmine",
        "Liam",
        "Isabella",
        "Noah",
        "Sophia",
        "Benjamin",
        "Olivia",
        "William",
        "Emma",
        "James",
        "Ava",
        "Alexander",
        "Mia",
        "Ethan",
        "Charlotte",
        "Jacob",
        "Amelia",
        "Michael",
        "Harper",
        "Elijah",
    ];
    let id = rand::thread_rng().gen_range(1..=100);
    let idx = rand::thread_rng().gen_range(0..names.len());
    let name = String::from(names[idx]);
    let current_time = Instant::now();
    (id as i32, name, current_time)
}

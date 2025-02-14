use std::sync::Mutex;
use std::thread;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(|| {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(1); //pushing 1 to the vector
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    let lock = NUMBERS.lock().unwrap();
    println!("{:#?}", lock);
}

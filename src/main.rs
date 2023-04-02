mod data;
use data::Data;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

fn main() {
    let a = Arc::new(Mutex::new(Data::new()));
    let a1 = a.clone();
    let a2 = a.clone();
    let t1 = thread::spawn(move || {
        let _ = a1.lock().unwrap().add_term("term".to_string());
        });
    let t2 = thread::spawn(move || {
        let _ = a2.lock().unwrap().add_term("another term".to_string());
    });

    t1.join();
    t2.join();

    let t3 = thread::spawn(move || {
        a.lock().unwrap().print_data();
    });
    t3.join();

    
    
}

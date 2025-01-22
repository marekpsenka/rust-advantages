use std::sync::{Arc, Mutex};

fn f(n_container: Arc<Mutex<i32>>) {
    let mut n = n_container.lock().expect("Lock is not poisoned");
    *n += 1;
}

fn main() {
    let n_container = Arc::new(Mutex::new(0i32));
    let container_clone = n_container.clone();
    let my_thread = std::thread::spawn(move || {
        f(container_clone);
    });
    _ = my_thread.join();
    let n = n_container.lock().expect("Lock not poisoned");
    println!("{}", *n);
}

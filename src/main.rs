//safe_code() and unsafe_code() was yoinked from https://doc.rust-lang.org/nomicon/races.html
//currently losing my mind, borrow checker won't let me borrow in unsafe.


use std::{thread, time::Duration};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;


fn unsafe_code(){
let data = vec![1, 2, 3, 4,5,6,7,8,9];

let idx = Arc::new(AtomicUsize::new(0));
let other_idx = idx.clone();


thread::spawn(move || {

    other_idx.fetch_add(10, Ordering::SeqCst);

    //this line breaks, borrow checker still gets real mad in unsafe
    //idx.fetch_add(10, Ordering::SeqCst);
});


if idx.load(Ordering::SeqCst) < data.len() {
    unsafe {
        idx.fetch_add(10, Ordering::SeqCst);

        //does some funny memory thing from double access, just prints a random ass number
        println!("{}", data.get_unchecked(idx.load(Ordering::SeqCst)));
    }
}

}


fn safe_code(){


let data = vec![1, 2, 3, 4];

let idx = Arc::new(AtomicUsize::new(0));
let other_idx = idx.clone();


thread::spawn(move || {

    other_idx.fetch_add(10, Ordering::SeqCst);
});


println!("{}", data[idx.load(Ordering::SeqCst)]);
}

fn main(){
    safe_code();
    unsafe_code();

    let wow = deadlock();
    thread::park();
    use std::{thread, time::Duration};
    thread::sleep(Duration::from_secs(10));
    println!("{}", wow);
    //println!("sleep over");
}

//static deta:Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
//static dota:Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

//IT WORKS WE HAVE A DEADLOCK IN SAFE RUST
fn deadlock() -> i32{
    let deta = Arc::new(Mutex::new(0));
    let dota = Arc::new(Mutex::new(0));

     unsafe{
        let sds = Arc::clone(&deta); let dsd = Arc::clone(&dota); 
     thread::spawn(move || { sds.lock(); dsd.lock();});
     let asa = Arc::clone(&deta); let sas = Arc::clone(&dota); 
     thread::spawn(move || { asa.lock(); sas.lock(); asa.lock(); });
     }

    for x in 0..4 {
        let (deta, dota) = (Arc::clone(&deta), Arc::clone(&dota));
        let handle = thread::spawn(move || {
            println!("spawned");
            thread::sleep(Duration::from_secs(1));
            if x % 2 == 0{
                let mut data = deta.lock().unwrap();
                *data += 1;
                //panic!();
                //this panic poisons the lock so nothing can use it which is neat, 
                println!("deta locked");
                thread::sleep(Duration::from_secs(10));
                let mut dota = dota.lock().unwrap();
                *dota = *data+1;
            }
            else{
                let mut dota = dota.lock().unwrap();
                *dota += 1;
                //panic!();
                println!("dota locked");
                thread::sleep(Duration::from_secs(10));
                let mut data = deta.lock().unwrap();
                *data = *dota + 1;
            }
            println!("{}", x);
            //thread::sleep(Duration::from_secs(5));
            //thread::unpark();
            // the lock is unlocked here when `data` goes out of scope.
        });
        //handle.join();
    }
    return 4;
}
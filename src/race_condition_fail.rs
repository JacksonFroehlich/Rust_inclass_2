//use tokio;


async fn sleep_1s_blocking(task: &str) {
    use std::{thread, time::Duration};
    println!("Entering sleep_1s_blocking({task})");
    thread::sleep(Duration::from_secs(1));
    println!("Returning from sleep_1s_blocking({task})");
    }


async fn do_it_safe<'a>(mut ve: Vec<i32>, ind:  i32){
    unsafe{
        ve[ind as usize] +=3;
       
}

}

   // #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
   #[tokio::main]
    async fn main() {
    let mut test = Vec::new();
    for x in 1..100
    {
        test.push(x);
    }
let mut num:i32 = 0;
    println!("Test 1: Run 2 async tasks sequentially");
    sleep_1s_blocking("Task 1").await;
    sleep_1s_blocking("Task 2").await;
    println!("Test 2: Run 2 async tasks concurrently (same thread)");
    tokio::join!(
    sleep_1s_blocking("Task 3"),
    sleep_1s_blocking("Task 4")
    );
    println!("Test 3: Run 2 async tasks in parallel");
    unsafe{
        let mut num:i32 = 0;
        
        let thing = tokio::task::spawn(do_it_safe( test, 0));
        let thing2 =     tokio::task::spawn(do_it_safe( test, 1));
        
        

    }

    
    use std::{thread, time::Duration};
    sleep_1s_blocking("Task 2").await;
    thread::sleep(Duration::from_secs(100));
    }
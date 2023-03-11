#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;
    use std::{thread, time::Duration};
    #[test]
    fn spanwn_and_main() {
        println!();
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn spanwn_and_main_with_join() {
        println!();
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(10));
            }
        });
        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }
}

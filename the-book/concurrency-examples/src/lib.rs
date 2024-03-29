#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;
    use std::{sync::mpsc, thread, time::Duration};
    #[test]
    fn spawn_and_main() {
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
    fn spawn_and_main_with_join() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(10));
            }
        });
        handle.join().unwrap();
        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn moved_spawn() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    #[test]
    fn single_producer_single_consumer() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }

    #[test]
    fn conversation_in_channel() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        });
        for received in rx {
            println!("Got: {received}")
        }
    }

    #[test]
    fn multiple_producer_single_consumer() {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        });
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("message"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        });
        for received in rx {
            println!("Got: {received}")
        }
    }
}

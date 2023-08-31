use std::thread;
use std::time::Duration;

const NUM_THREADS: usize = 10;

fn thread_func(id: usize) -> &'static str {
    for i in 0..5 {
        println!("id = {}, i = {}", id, i);
        thread::sleep(Duration::from_secs(1));
    }
    "finished!"
}

// 2.2.1.1　スレッドの生成と終了待ち合わせ

#[allow(dead_code)]
fn thread_main() {
    let mut v = Vec::with_capacity(NUM_THREADS);
    for i in 0..NUM_THREADS {
        let handle = thread::spawn(move || thread_func(i));
        v.push(handle);
    }
    for handle in v {
        if let Ok(result) = handle.join() {
            println!("msg = {}", result);
        } else {
            eprintln!("pthread_join");
        }
    }
}

// 2.2.1.2　デタッチスレッド
// RustのDropトレイトはCでいうところの、デタッチスレッドあるいは自動的にメモリを開放するRAII。
// RAIIとはそのクラスが（主にコンストラクタで確保して）所有するリソースを、デストラクタで解放することによってリソース管理を自動化するテクニック
#[allow(dead_code)]
fn detach_thread() {
    // バックグランドタスクのような例外はあるが基本的にスコープを抜けると自動開放される
    let attr = thread::spawn(move || thread_func(1));
    attr.join().expect("Failed to join thread");
    thread::sleep(Duration::from_secs(7));
}

fn main() {}

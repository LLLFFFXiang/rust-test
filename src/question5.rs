// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 模拟下载函数
fn download(url: &str) -> String {

    let duration = Duration::from_millis(100 * (url.len() as u64 % 5));
    thread::sleep(duration);
    format!("{} - 下载完成 (耗时 {}ms)", url, duration.as_millis())
}

fn main() {
    // 要下载的URL列表
    let urls = vec![
        "https://example.com/file1",
        "https://example.com/file2",
        "https://example.com/file3",
    ];

    // 创建消息通道（多生产者，单消费者）
    let (sender, receiver) = mpsc::channel();

    // 创建线程池
    let mut handles = vec![];

    for url in urls {
        // 克隆发送端到每个线程
        let sender = sender.clone();
        
        // 创建线程
        let handle = thread::spawn(move || {
            // 执行下载任务
            let result = download(url);
            // 发送结果到主线程
            sender.send(result).unwrap();
        });

        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 主线程接收并打印结果
    println!("下载结果:");
    for _ in 0..urls.len() {
        let result = receiver.recv().unwrap();
        println!("{}", result);
    }
}
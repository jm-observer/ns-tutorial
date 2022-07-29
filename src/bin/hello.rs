use std::time::Duration;

/// 用于测试内存的占用情况
///
fn main() {
    let mut count = 0;
    // 不能改成数组，因为数组是在栈上分配的，极容易造成段错误
    // 不能为0u8，会被优化
    let mut data = vec![2u8; 1024 * 1024 * 1024 * 8];
    loop {
        count += 1;
        println!("hello {}", count);
        std::thread::sleep(Duration::from_secs(10));
        if count > 10 {
            break;
        }
    }
    println!(
        "data.len={}, last={:?}",
        data.len(),
        data.get(data.len() - 1)
    );
}

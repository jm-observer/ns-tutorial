use ns_tutorial::check_os;
use std::time::Duration;

#[tokio::main]
async fn main() {
    if let Err(e) = check_os().await {
        println!("check os error: {:?}", e);
    }

    let mut count = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
        println!("count: {}", count);
        count += 1;
    }
}

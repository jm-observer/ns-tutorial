pub mod local_socket;
use crate::local_socket::client;
use anyhow::Result;
use heim::*;

pub async fn check_os() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        use nix::unistd::*;
        println!("uid: {}, gid: {}", getuid(), getgid());
        println!("pid: {}", getpid());
        let mut hostname = [0u8; 500];
        println!("hostname: {:?}", gethostname(hostname.as_mut_slice())?);
        println!("cwp: {:?}", getcwd()?);

        match std::fs::read_to_string("/proc/mounts") {
            Err(e) => println!("/proc/mounts read {:?}", e),
            Ok(file) => {
                println!("/proc/mounts: {}", file);
            }
        }
    }

    // unsafe {
    //     println!("cpu: {}", sched_getcpu());
    // }
    match cpu::logical_count().await {
        Ok(count) => println!("cpu logical count {}", count),
        Err(e) => println!("logical_count {:?}", e),
    }
    match cpu::physical_count().await {
        Ok(Some(count)) => println!("cpu physical count {}", count),
        Ok(None) => println!("cpu physical count none"),
        Err(e) => println!("physical_count {:?}", e),
    }
    // println!(
    //     "cpu logical count {}, physical count: {}",
    //     heim_cpu::logical_count().await.unwrap(),
    //     heim_cpu::physical_count().await.unwrap().unwrap()
    // );
    // let partitions = heim_disk::partitions().await.unwrap();
    // while let Some(Ok(item)) = partitions.next().await {
    //     println!("partition: {:?}", item);
    // }

    match memory::memory().await {
        Ok(count) => println!("memory {:?}", count),
        Err(e) => println!("memory {:?}", e),
    }

    match memory::swap().await {
        Ok(count) => println!("swap {:?}", count),
        Err(e) => println!("swap {:?}", e),
    }

    if let Err(e) = client() {
        println!("ipc-local-socket error: {:?}", e);
    }

    let key = "RUST_BACKTRACE";
    match std::env::var_os(key) {
        Some(val) => println!("{key}: {val:?}"),
        None => println!("{key} is not defined in the environment."),
    }
    Ok(())
}

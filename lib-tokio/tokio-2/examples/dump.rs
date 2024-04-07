use std::time::Duration;

#[cfg(all(
    tokio_unstable,
    tokio_taskdump,
    target_os = "linux",
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    async fn dump_or_quit() {
        use tokio::time::{timeout, Duration};
        let handle = tokio::runtime::Handle::current();

        let mut cnt = 1;

        loop {
            if timeout(Duration::from_secs(2), handle.dump()).await.is_ok() {
                println!("Got tasks {cnt}");
                cnt += 1;
            } else {
                println!("Task dumping timed out. Use a native debugger (like gdb) to debug the deadlock.");
            }
        }
    }

    let task_3 = tokio::spawn(async {
        let mut cnt = 1;
        loop {
            println!("Sleep {cnt} ...");
            tokio::time::sleep(Duration::from_millis(1)).await;
            println!("Wake up");
            cnt += 1;
        }
    });

    tokio::select!(
        _ = dump_or_quit() => {},
        _ = task_3 => {},
    );

    Ok(())
}
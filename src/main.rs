#[tokio::main(flavor = "current_thread")]
async fn main() {
    let result = tokio::select! {
        err = stdin_lines() => err,
        err = error() => err,
    };

    match result {
        Ok(_) => unreachable!(),
        Err(e) => println!("error: {}", e),
    }
}

async fn stdin_lines() -> Result<std::convert::Infallible, Box<dyn std::error::Error>> {
    use tokio::io::AsyncBufReadExt;

    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    loop {
        let _ = lines.next_line().await?.unwrap();
    }
}

async fn error() -> Result<std::convert::Infallible, Box<dyn std::error::Error>> {
    println!("erroring in 1 second...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("erroring now");
    Err("boom".into())
}

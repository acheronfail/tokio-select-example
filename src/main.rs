#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<std::convert::Infallible, Box<dyn std::error::Error>> {
    tokio::select! {
        err = stdin_lines() => err,
        err = error() => err,
    }
}

async fn stdin_lines() -> Result<std::convert::Infallible, Box<dyn std::error::Error>> {
    use tokio::io::AsyncBufReadExt;

    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    loop {
        println!("next line");
        let line = lines.next_line().await?.unwrap();
        println!("recv line: {}", line);
    }
}

async fn error() -> Result<std::convert::Infallible, Box<dyn std::error::Error>> {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Err("boom".into())
}

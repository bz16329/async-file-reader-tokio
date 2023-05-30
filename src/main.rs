use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let file_contents = read_file_contents("README.md").await;
    match file_contents {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}

async fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

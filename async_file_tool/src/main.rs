use tokio::fs::{self, File};
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let input_path = "input.txt";
    let output_path = "output.txt";

    // Read the content from the input file asynchronously.
    let content = fs::read_to_string(input_path).await?;
    println!("Read from {}: {}", input_path, content);

    // Open or create the output file.
    let mut output = File::create(output_path).await?;

    // Write the content to the output file asynchronously.
    output.write_all(content.as_bytes()).await?;
    println!("Written to {}: {}", output_path, content);

    Ok(())
}

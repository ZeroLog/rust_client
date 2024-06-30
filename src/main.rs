
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
 
fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:4545")?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);
    let stdin = io::stdin();
    let mut input = String::new();
    let mut response = String::new();
     
    loop {

        input.clear();
        stdin.read_line(&mut input)?;
        writer.write_all(input.as_bytes())?;
        writer.flush()?;
       
        response.clear();
        reader.read_line(&mut response)?;
        if response.is_empty() {
            break;
        }
        println!("Server: {}", response.trim());
       
    }

    Ok(())
}

use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:4545")?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);
    let stdin = io::stdin();
    let mut input = String::new();
    let mut response = String::new();
    let mut array: [Vec<i16>; 3] = [vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
    response = read_data(&mut reader)?;
    let r: i8 = response.trim().parse().unwrap();
    response.clear();
    print_array(&array);

    loop {
        if r == 2 {
            println!("Ваш ход");
            input = write_data(&stdin, &mut writer)?;
            println!("Ждите хода соперника");
            response = read_data(&mut reader)?;
            println!("Server: {}", response.trim());
        }

        if r == 1 {
            response = read_data(&mut reader)?;
            println!("Server: {}", response.trim());
            println!("Ваш ход");
            input = write_data(&stdin, &mut writer)?;
        }
    }

    Ok(())
}
fn print_array(array: &[Vec<i16>; 3]) {
    for row in array.iter() {
        for val in row.iter() {
            print!("{} ", val);
        }
        println!();
    }
}
fn read_data(reader: &mut BufReader<TcpStream>) -> std::io::Result<String> {
    let mut response = String::new();
    reader.read_line(&mut response)?;
    Ok(response)
}

fn write_data(stdin: &io::Stdin, writer: &mut BufWriter<TcpStream>) -> std::io::Result<String> {
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    writer.write_all(input.as_bytes())?;
    writer.flush()?;
    Ok(input.trim().to_string())
}

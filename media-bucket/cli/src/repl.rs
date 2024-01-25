use std::{
    error::Error,
    io::stdout,
    io::{stdin, Write},
};

use libmb::Bucket;

async fn run_line(cmd: &str, bucket: &mut Bucket) -> Result<(), Box<dyn Error>> {
    match cmd {
        "posts" => {}
        _ => println!("Uknown command '{cmd}'"),
    }
    Ok(())
}

pub async fn start_repl(mut bucket: Bucket) -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    loop {
        line.clear();
        print!("> ");
        stdout().flush()?;
        match stdin().read_line(&mut line) {
            Ok(0) => break Ok(()),
            Ok(size) => run_line(line[0..size].trim(), &mut bucket).await?,
            Err(e) => break Err(e.into()),
        }
    }
}

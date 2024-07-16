use std::io::{BufRead, Write};

use clap::Parser;

#[derive(Parser)]
struct Command {
    #[clap(short, long)]
    pub lines: usize,

    #[clap(short, long)]
    pub prefix: String,

    pub input: String,
}

fn get_file_name(prefix: &str, count: usize) -> String {
    format!("{}-{}.csv", prefix, count)
}

fn main() {
    let args = Command::parse();
    println!("lines: {}", args.lines);
    println!("prefix: {}", args.prefix);
    println!("input: {}", args.input);

    // load input file as stream
    let file = std::fs::File::open(&args.input).expect("file not found");

    // csv headers is first line of file
    let file_stream = std::io::BufReader::new(file);

    let mut lines = file_stream.lines();
    let headers = lines
        .next()
        .expect("file is empty")
        .expect("error reading file");

    let mut file_count = 0;

    loop {
        let file_name = get_file_name(&args.prefix, file_count);
        let file = std::fs::File::create(file_name.clone()).expect("file not found");
        let mut file = std::io::BufWriter::new(file);

        println!("building file: {}", file_name);
        // write headers
        file.write_all(headers.as_bytes())
            .expect("error writing file");

        file.write_all(b"\n").expect("error writing file");

        for _ in 0..args.lines {
            match lines.next() {
                Some(line) => {
                    let line = line.expect("error reading file");
                    file.write_all(line.as_bytes()).expect("error writing file");
                    file.write_all(b"\n").expect("error writing file");
                }
                None => {
                    file.flush().expect("error writing file");
                    return;
                }
            }
        }
        file.flush().expect("error writing file");
        file_count += 1;
        println!("created file: {}", file_name);
    }
}

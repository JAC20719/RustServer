use std::{env, io::{BufRead, BufReader, Write}, net::TcpStream, path::PrefixComponent};
use::std::fs;
use std::io::Bytes;
use::std::str;
use std::net::TcpListener;


fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

    for stream in listener.incoming() {
        println!("Connection Established!");
        let stream = stream.unwrap();
        
        handle_connection(stream);
    }


    /* let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("a valid file to read from");

    let mut contents_as_bytes: String = String::from("");

    for line in contents.lines() {
        let line_bytes = line.as_bytes().to_vec();
        println!("Bytes for {}={}: ", line, line_bytes.len());

        for word in line.split(" "){
            let word_bytes = word.as_bytes().to_vec();
            println!("  Bytes for {}={}: ", word, word_bytes.len());

            for char in word.chars().into_iter() {
                println!("      character: {}", char);
    
                let char_bytes = char.to_string().as_bytes().to_vec();
                print!("        ");
                for byte in char_bytes.iter() {
                    //let byte_ptr: *const u8 = byte as *const u8;
                    print!("{:08b} - ", byte);
                    if (contents_as_bytes.is_empty()) {
                        contents_as_bytes = format!("{:08b}",byte);
                    } else {
                        contents_as_bytes = contents_as_bytes + " " + &format!("{:08b}",byte);
                    }
                    
                }

                contents_as_bytes = contents_as_bytes +  " " +"00100000";
                println!("");
            }
        }  
    }

    let output_path = "output.bin";
    println!("File converted to binary: {}", contents_as_bytes);
    match fs::write(output_path, contents_as_bytes) {
        Ok(_) => println!("Successfully converted file to binary!"),
        Err(_) => println!("Something went wrong while creating file!"),
    };

    let bin_contents = fs::read_to_string(output_path).expect("can read output file");
    println!("Reading output file: {}", output_path);
    for line in bin_contents.lines() {
        println!("{}",line);
        let mut byte_vec = Vec::new();

        for word in line.split(" ") {
            println!("Word: {}", word);

            
            byte_vec.push(u8::from_str_radix(word, 2).expect("Not a binary number!"));

        }

        byte_vec.iter().for_each(move |b| {
            println!("{}",b);
        });

        println!("Converted string: {}",str::from_utf8(&byte_vec).expect("Not valid utf8!"));
    } */
    
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = get_html(); //fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_html() -> &'static str {
    return "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>Hello!</title></head><body><h1>Hello!</h1><p>Hi from Rust</p></body></html>";
}

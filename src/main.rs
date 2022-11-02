use std::fmt::{Debug, Formatter};
use std::io;
use std::io::{Error, Write};
use zstd;
use zstd::Encoder;

// This function use the convenient `copy_encode` method
fn compress(level: i32) {
    zstd::stream::copy_encode(io::stdin(), io::stdout(), level).unwrap();
}

fn decompress() {
    zstd::stream::copy_decode(io::stdin(), io::stdout()).unwrap();
}

fn compress_manually(level: i32) {
    let mut encoder = zstd::stream::Encoder::new(io::stdout(), level).unwrap();
    io::copy(&mut io::stdin(), &mut encoder).unwrap();
    encoder.finish().unwrap();
}


fn main() {
    let level = 0;
    let mut message: Vec<u8> = [0; 1024*1024].to_vec();
    message[1] = 1;
    let mut message_readable = message.as_slice();
    println!("message: {:?}", &message[..100]);

    // if vec is initialised at length 0, I get the error: "writer will not accept any more data"
    let mut output_buf = [0; 1024*1024].to_vec();
    let output_buf_writable = output_buf.as_mut_slice();

    let mut encoder = zstd::stream::Encoder::new(output_buf_writable, level).unwrap();
    let bytes_read = io::copy(&mut message_readable, &mut encoder).unwrap();

    let bytes_written = encoder.get_bytes_written();
    println!("bytes_read: {:?}", bytes_read);
    println!("bytes_written: {:?}", bytes_written);
    let result = encoder.finish().unwrap();
    println!("result.len(): {:?}", result.len());
}

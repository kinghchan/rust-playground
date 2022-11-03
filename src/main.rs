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
    // ENCODE //
    let level = 0;
    let mut message: Vec<u8> = [0; 1024*1024].to_vec();
    message[1] = 1;
    let mut message_readable = message.as_slice();
    println!("Message: {:?} ...", &message[..25]);
    println!("Message length: {:?} ...", message.len());

    // if vec is initialised at length 0, I get the error: "writer will not accept any more data"
    let mut output_buf = [0; 1024*1024].to_vec();
    let output_buf_writable = output_buf.as_mut_slice();

    let mut encoder = Encoder::new(output_buf_writable, level).unwrap();
    io::copy(&mut message_readable, &mut encoder).unwrap();

    encoder = encoder.finish_and_return_self();

    let bytes_written = encoder.get_bytes_written();
    // println!("bytes_read: {:?}", bytes_read);
    println!("bytes_written: {:?}", bytes_written);

    println!("Compressed result: {:?}", &output_buf[..bytes_written]);
    // looks like I'm 9 bytes short
    println!("Compressed result (extended): {:?}", &output_buf[..bytes_written]);

    // DECODE //
    let mut reconstructed_buf = [0; 1024*1024].to_vec();
    let reconstructed_buf_writable = reconstructed_buf.as_mut_slice();

    zstd::stream::copy_decode(&output_buf[..bytes_written], reconstructed_buf_writable).unwrap();

    println!("Decompressed result: {:?} ...", &reconstructed_buf[..10]);
    println!("Decompressed length: {:?}", reconstructed_buf.len());
}

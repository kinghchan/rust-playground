use std::fmt::{Debug, Formatter};
use std::io;
use std::io::{Error, Write};
use std::time::{SystemTime, UNIX_EPOCH};
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

fn decompress_manually(level: i32) {
    let mut decoder = zstd::stream::Decoder::new(io::stdin()).unwrap();
    io::copy(&mut decoder, &mut io::stdout()).unwrap();
    decoder.finish();
}

struct ZstdWritableBuffer<'a> {
    writer: &'a mut [u8],
    bytes_written: usize,
}

impl<'a> Write for ZstdWritableBuffer<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_written = self.writer.write(buf).expect("Error writing");
        self.bytes_written += bytes_written;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

pub fn get_current_ts_ns() -> u64 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    time
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
    // same if non-zero but still not large enough
    let mut output_buf = [0; 1024*1024].to_vec();
    let output_buf_writable = output_buf.as_mut_slice();

    let mut writer = ZstdWritableBuffer {
        writer: output_buf_writable,
        bytes_written: 0,
    };

    let mut encoder = Encoder::new(writer, level).unwrap();
    io::copy(&mut message_readable, &mut encoder).unwrap();


    writer = encoder.finish().expect("Error finishing");
    let bytes_written = writer.bytes_written;
    // println!("bytes_read: {:?}", bytes_read);
    println!("bytes_written: {:?}", bytes_written);

    println!("Compressed result: {:?}", &output_buf[..bytes_written]);
    // looks like I'm 9 bytes short
    println!("Compressed result (extended): {:?}", &output_buf[..bytes_written]);

    // DECODE //

    let ts_before = get_current_ts_ns();
    let mut reconstructed_buf = [0; 1024*1024].to_vec();
    let reconstructed_buf_writable = reconstructed_buf.as_mut_slice();

    let mut decompress_writer = ZstdWritableBuffer {
        writer: reconstructed_buf_writable,
        bytes_written: 0,
    };


    let mut decoder = zstd::stream::Decoder::new(&output_buf[..bytes_written]).unwrap();
    io::copy(&mut decoder, &mut decompress_writer).unwrap();

    let bytes_written = decompress_writer.bytes_written;

    let ts_after = get_current_ts_ns();
    let diff = ts_after - ts_before;
    println!("diff: {:?} (us)", diff / 1000);

    println!("Decompressed length: {:?}", bytes_written);
    println!("Decompressed result: {:?} ...", &reconstructed_buf[..10]);

}

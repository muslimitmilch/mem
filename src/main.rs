use  std::io;
use  std::io::Read;
use  std::io::stdout;
use  termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
} 

fn main() {
    let _stdout_raw_mode = stdout().into_raw_mode().unwrap();

    for byte in io::stdin().bytes() {
        let byte = byte.unwrap();
        let character = byte as char;
        println!("{}\r", character);

        if byte == to_ctrl_byte('q') {break}
    }
}

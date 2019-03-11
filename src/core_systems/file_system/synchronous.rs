use std::path::Path;
use std::fs;
use std::io;

pub fn read_file_to_string<P: AsRef<Path>>(filepath: P) -> Result<String, io::Error> {
    let fp = fs::File::open(filepath);


}

pub fn read_file_to_buf<P: AsRef<Path>>(filepath: P) {

}
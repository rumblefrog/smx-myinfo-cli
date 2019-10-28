use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Cursor, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use smxdasm::file::SMXFile;

fn main() {
    let path: PathBuf = PathBuf::from(std::env::args().nth(1).expect("No File Provided"));

    let mut file = File::open(path).unwrap();

    let mut data = Vec::new();

    file.read_to_end(&mut data).unwrap();

    let dasm = SMXFile::new(data).unwrap();

    let file = dasm.borrow();

    let data = file.data.as_ref().unwrap().get_data_vec();

    let pubvars = file.pubvars.as_ref().unwrap();

    let mut var_iter = pubvars.entries().into_iter();

    let entry = var_iter.find(|e| e.name == "myinfo").unwrap();

    let mut cursor = Cursor::new(data);

    cursor.seek(SeekFrom::Start(entry.address as u64)).unwrap();

    let cells: [i32; 5] = [
        cursor.read_i32::<LittleEndian>().unwrap(),
        cursor.read_i32::<LittleEndian>().unwrap(),
        cursor.read_i32::<LittleEndian>().unwrap(),
        cursor.read_i32::<LittleEndian>().unwrap(),
        cursor.read_i32::<LittleEndian>().unwrap(),
    ];

    let mut read_string = |index: usize| -> String {
        let mut buf = [0; 1];
        let mut str_vec: Vec<u8> = Vec::with_capacity(256);

        cursor.seek(SeekFrom::Start(cells[index] as u64)).unwrap();

        loop {
            cursor.read_exact(&mut buf).unwrap();

            if buf[0] == 0 {
                break;
            } else {
                str_vec.push(buf[0]);
            }
        }

        String::from_utf8_lossy(&str_vec[..]).into_owned()
    };

    println!("Name: {}", read_string(0));
    println!("Description: {}", read_string(1));
    println!("Author: {}", read_string(2));
    println!("Version: {}", read_string(3));
    println!("URL: {}", read_string(4));
}

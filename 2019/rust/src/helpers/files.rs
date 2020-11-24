use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> String {
    let mut contents = String::new();
    let file_result = File::open(path);
    match file_result {
        Ok(mut file) => {
            let _result = file.read_to_string(&mut contents);
        },
        Err(e) => println!("File access error: {:?}", e),
    }
    contents
}

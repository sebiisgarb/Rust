use std::fs::File;

fn main() {
    match File::create("foo.txt") {
        Ok(val) = val,
        Err(err) = (err) {
            println!("File creation failed: {:?}", err);
        }
    }
}
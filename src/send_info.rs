
pub mod SendInfo {
    use std::fs::*;
    use std::io::ErrorKind;
    use std::io::Write;
    use std::fs::OpenOptions;
    use mail_send::{mail_builder::*, SmtpClientBuilder};

const FILENAME: &str = "items.txt";

fn open_file(filename: &str) -> File {
    let file = match OpenOptions::new().write(true).append(true).open(filename) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                match File::create(filename) {
                    Ok(file) => file,
                    Err(_) => panic!("Can't create {filename} file"),
                }
            }
            else {
                panic!("Problems with opening {filename} file");
            }
        }
    };

    file
}

pub fn write_item(item: String) {
    let mut f = open_file(FILENAME);

    match write!(&mut f, "{}\n" ,item) {
        Ok(_) => (),
        Err(err) => eprintln!("Error in writing to file: {err:?}"),
    }
}

}
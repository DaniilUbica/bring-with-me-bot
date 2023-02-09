
pub mod send_info {
    use std::fs;
    use std::fs::*;
    use std::fs::OpenOptions;
    use std::io::ErrorKind;
    use std::io::Write;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

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

    pub fn send_items_info() {
        let items = fs::read_to_string(FILENAME).expect("Can't read file");

        let email = Message::builder()
        .from("Botik <sender@gmail.com>".parse().unwrap())
        .to("<receiver@gmail.com@gmail.com>".parse().unwrap())
        .subject("Unknown items")
        .body(String::from(format!("Unknown items:\n {items}")))
        .unwrap();

        let creds = Credentials::new("sender@gmail.com".to_string(), "pass".to_string());

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }

}
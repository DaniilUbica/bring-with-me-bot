
pub mod Database {

    use std::fs::File;

    const TABLE_NAME: &str = "items";
    const DATABASE_NAME: &str = "tg_bot_db.db";

    fn connect_to_db() -> sqlite::Connection {
        let connection = match sqlite::open(DATABASE_NAME) {
            Ok(connection) => connection,
            Err(_) => {
                File::create(DATABASE_NAME).expect("Can't create database file");
                let connection = sqlite::open(DATABASE_NAME).unwrap();

                connection
            }
        };

        connection
    }

    pub fn create_table() {
        let query = format!("CREATE TABLE IF NOT EXISTS {TABLE_NAME} (rus_name TEXT, eng_name TEXT, rus_info TEXT, eng_info TEXT)");

        let connection = connect_to_db();

        connection.execute(query).expect("Can't execute query");
    }

    pub fn add_record(columns: Vec<&str>) -> bool {
        let rus = columns[0];
        let eng = columns[1];
        let rus_info = columns[2];
        let eng_info = columns[3];

        let query = format!("INSERT INTO {TABLE_NAME} VALUES ('{rus}', '{eng}', '{rus_info}', '{eng_info}')");

        let connection = connect_to_db();

        match connection.execute(query) {
            Ok(_) => true,
            Err(err) => {
                eprintln!("Error in adding the record: {err}");
                false
            }
        }
    }

    pub fn get_record(name: String) -> String {
        let query = format!("SELECT DISTINCT rus_info FROM {TABLE_NAME} WHERE rus_name = '{name}'");
        let mut answer = String::new();

        let connection = connect_to_db();

        connection
        .iterate(query, |pairs| {
            for &(_name, value) in pairs.iter() {
                let value = match value {
                    Some(val) => val,
                    None => { 
                        eprintln!("Can't get value from database");
                        " "
                    }
                };

                let s = format!("{}", value);
                answer = s;
            }
            true
        })
        .expect("Error in request to database");

        if !answer.is_empty() {
            return answer;
        }
        else {
            let query = format!("SELECT DISTINCT eng_info FROM {TABLE_NAME} WHERE eng_name = '{name}'");

            let connection = connect_to_db();

            connection
            .iterate(query, |pairs| {
                for &(_name, value) in pairs.iter() {
                    let value = match value {
                        Some(val) => val,
                        None => { 
                            eprintln!("Can't get value from database");
                            " "
                        }
                    };

                    let s = format!("{}", value);
                    answer = s;
                }
                true
            })
            .expect("Error in request to database");
        }
        answer

    }

}

#[cfg(test)]
mod tests {
    use crate::Database::{get_record, add_record};

    #[test]
    fn add_record_check_passed() {
        assert!(add_record(["qwe", "zxc", "asd", "cvb"].to_vec()));
    }

    #[test]
    fn get_record_check_passed() {
        let record = get_record("knife".to_string());
        assert!(!record.is_empty());
    }
    
    #[test]
    fn get_record_check_failed() {
        let record = get_record("brush".to_string());
        assert!(record.is_empty());
    }
}
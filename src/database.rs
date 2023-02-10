
pub mod Database {

    use std::fs::File;

    const TABLE_NAME: &str = "items";
    const DATABASE_NAME: &str = "items.db";

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
        let query = format!("CREATE TABLE IF NOT EXISTS {TABLE_NAME} (rus_name TEXT, eng_name TEXT, rus_info TEXT, eng_info TEXT,
             rus_allowed TEXT, eng_allowed TEXT)");

        let connection = connect_to_db();

        connection.execute(query).expect("Can't execute query");
    }

    pub fn add_record(columns: Vec<&str>) -> bool {
        let rus = columns[0];
        let eng = columns[1];
        let rus_info = columns[2];
        let eng_info = columns[3];
        let rus_allowed = columns[4];
        let eng_allowed = columns[5];

        let query = format!("INSERT INTO {TABLE_NAME} VALUES ('{rus}', '{eng}', '{rus_info}', '{eng_info}', '{rus_allowed}', '{eng_allowed}')");

        let connection = connect_to_db();

        match connection.execute(query) {
            Ok(_) => true,
            Err(err) => {
                eprintln!("Error in adding the record: {err}");
                false
            }
        }
    }

    fn get_rus_info(name: String) -> String {
        let query = format!("SELECT DISTINCT rus_info, rus_allowed FROM {TABLE_NAME} WHERE rus_name = '{name}'");
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
                let s = format!(" {}", value);
                answer += &s;
            }
            true
        })
        .expect("Error in request to database");

        if !answer.is_empty() {

            let v: Vec<&str> = answer.split_whitespace().collect();

            answer = v[0].to_owned() + "\n\nКороче говоря: " + v[1];

        }

        answer
    }

    fn get_eng_info(name: String) -> String {
        let query = format!("SELECT DISTINCT eng_info, eng_allowed FROM {TABLE_NAME} WHERE eng_name = '{name}'");
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
                let s = format!(" {}", value);
                answer += &s;
            }
            true
        })
        .expect("Error in request to database");

        if !answer.is_empty() {

            let v: Vec<&str> = answer.split_whitespace().collect();

            answer = v[0].to_owned() + "\n\nIn short: " + v[1];
        }

        answer
    }

    pub fn get_record(name: String) -> String {
        let mut ret = get_rus_info(name.clone());
        
        if ret.is_empty() {
            ret = get_eng_info(name);
        }

        ret
    }

    pub fn get_item_names(name: &String) -> (String, String) {
        let mut rus_name = String::new();
        let mut eng_name = String::new();

        let query = format!("SELECT DISTINCT eng_name FROM {TABLE_NAME} WHERE rus_name = '{name}'");

        let connection = connect_to_db();

        connection
        .iterate(query, |pairs| {
            for &(_name, value) in pairs.iter() {
                eng_name = match value {
                    Some(val) => val.to_string(),
                    None => { 
                        eprintln!("Can't get value from database");
                        " ".to_string()
                    }
                };
                rus_name = name.to_string();
            }
            true
        })
        .expect("Error in request to database");

        if !eng_name.is_empty() {
            (rus_name, eng_name)
        }
        
        else {
            let query = format!("SELECT DISTINCT rus_name FROM {TABLE_NAME} WHERE eng_name = '{name}'");
    
            connection
            .iterate(query, |pairs| {
                for &(_name, value) in pairs.iter() {
                    rus_name = match value {
                        Some(val) => val.to_string(),
                        None => { 
                            eprintln!("Can't get value from database");
                            " ".to_string()
                        }
                    };
                    eng_name = name.to_string();
                }
                true
            })
            .expect("Error in request to database");

            (rus_name, eng_name)
        }

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
use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);

    let key = args.next().expect("Key was not there");
    let val = args.next().unwrap();

    let mut database = Database::new().expect("Creating db failed!");
    database.insert(key, val);

    println!("Database currently has:\n");
    database.show();
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let val = chunks.next().expect("No val!");
            map.insert(key.to_owned(), val.to_owned());
        }

        Ok(Database {map})
    }

    // Add key and value to the database
    fn insert(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }

    // Write database to a text file
    fn flush(&self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, val) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(val);
            contents.push('\n');
        }

        std::fs::write("kv.db", contents)
    }

    // Print the contents of the database
    fn show(&self) {
        for pairs in &self.map {
            println!("  KEY: {}, VALUE: {}", pairs.0, pairs.1);
        }
    }
}

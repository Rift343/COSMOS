use std::{file, fs::File, io::{Read, Seek}};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn scheduler (mut json_file:&File){
    json_file.rewind().expect("Rewind error==> Can't reset de cursor of the File");
    println!("You passed the rewind");
    let mut buffer = Vec::new();
    json_file.read_to_end(&mut buffer).expect("Read to end error");
    let mut str_json :String = String::new();
    for i in buffer{
        str_json.push(i as char);
    }
    let parse_json=json::parse(&str_json.to_string()).unwrap();
    parse_json.dump();
    println!("{}",parse_json["table"]);
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_on_create_file(){
        {
            let mut fichier_test:std::fs::File = File::create("test.txt").expect("Can't create a file");
            scheduler(&fichier_test);
            let data = b"hello world";
            fichier_test.write_all(data).expect("Can't write in this file");
            scheduler(&fichier_test);
        }
        let mut fichier_test:std::fs::File = File::open("test.txt").expect("Can't open the file");
        scheduler(&fichier_test);
        let mut buf = [1];
        let mut _a = fichier_test.read(&mut buf).expect("Can't read a file");
        scheduler(&fichier_test);
    }

    #[test]
    fn test_on_json(){
        let fichier_json_test:std::fs::File = File::open("semantique.json").expect("Error ==> Can't read the JSON file");
        scheduler(&fichier_json_test);
    }
}

use std::{fs::File, io::{Read, Seek}};
use std::path::Path;


struct Attribute {
    name: String,
    table: String,
}

struct CSVFile{
    name:String,
    descriptor:File
}

impl CSVFile {
    fn reset_descriptor(&mut self){
        &self.descriptor.rewind().expect("Rewind error==> Can't reset de cursor of the File");
    }
    fn print_csv_file(&self){
        println!("{}",&self.name);
    }
}

fn csv_read_by_ligne(fichier:&mut CSVFile){
    //TO DO
}

fn csv_read_by_columns(fichier:&mut CSVFile)/*->CSVFile*/{
    
    // TO DO
}

fn proj(full_relation:&mut CSVFile,attribute_liste:Vec<Attribute>,project_relation:&File)/*->&File*/{
    &full_relation.reset_descriptor();
    csv_read_by_columns(full_relation);

    //TO DO
}

fn open_relation(pathcsv:String,name1:String)->CSVFile{
    let file:CSVFile = CSVFile { name: name1, descriptor: File::open(Path::new(&pathcsv)).expect("Error ==> can't open this file") };
    return file;
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use super::*;
    #[test]
    fn test1(){
        let a1 = open_relation("../data/CSV/personnetest.csv".to_string(), "R1".to_string());
        a1.print_csv_file();
    }
}
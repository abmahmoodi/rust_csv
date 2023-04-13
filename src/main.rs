use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::ffi::OsStr;
use std::error::Error;

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn read_csv(file_path: String, id: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", &record[2]);
        if &record[2] == id {
            println!("Hello")
        }
    }
    Ok(())
}

fn main() {
    let file_path = "file_list.txt";
    let lines = read_lines(file_path.to_string());
    for line in lines {
        let path = line.unwrap();
        let file_path = std::path::Path::new(&path);
        // println!("{}", file_path.display());
        let ext = file_path.extension().and_then(OsStr::to_str);

        if ext != None
        {
            let parts: Vec<_> = path.split("/").collect();
            // println!("{}", parts[2]);
            let id = parts[2];
            let file_csv = "id-folder-map.csv";

            let result = read_csv(file_csv.to_string(), id.to_string());
            println!("{:?}", result.unwrap());
        }
    }




}

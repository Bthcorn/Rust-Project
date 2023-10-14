

use csv::{Error, Reader, StringRecord};

use std::{
    borrow::Borrow,
    fs::File,
    io::{Write, Read},
    path::{Path, PathBuf},
    string::String,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
#[derive(Debug)]
pub struct Csv {
    header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug)]
pub struct ProcessingUnit {
    input: PathBuf,
    output: PathBuf,
}

fn main() -> Result<(), Error> {
    // Create a CSV reader from a file or any other source (e.g., a string)
    let input = "name,age\nAlice,30\nBob,25\nCharlie,35";
    let mut rdr = Reader::from_reader(input.as_bytes());
    // println!("{:?}", rdr);
    let headers = rdr.headers()?.clone();
    let headers_v: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
    let processing_unit = ProcessingUnit {input: PathBuf::from("offers-1000.csv"), output: PathBuf::from("output.json")};
    // println!("{:?}", headers);
    // Iterate over each record (row) in the CSV
    for result in rdr.records() {
        // Each record is a Result<StringRecord, Error>
        let record = result?;
        
        // Access fields within the record
        // let name = &record[0];
        // let age = &record[1];
        
        let x = convert_line(&headers_v, &record);
        // println!("Name: {}, Age: {}", name, age);
        println!("{:?}", x)
    }
    convert_data(&processing_unit);

    Ok(())
}

pub fn convert_line(headers: &[String], record: &StringRecord) -> String {
    let mut line = "{".to_owned();
    headers.iter().enumerate().for_each(|(i, h)| {
        let value = (record.get(i).unwrap()).to_string();
        line.push('"');
        line.push_str(h);
        line.push_str("\":\"");
        line.push_str(&value.replace('\"', "\\\""));
        line.push_str("\",");
    });

    let mut nline = line[0..line.len() - 1].to_string();
    nline.push_str("}\n");
    nline
}

pub fn convert_data(processing_unit: &ProcessingUnit) {
    if !Path::exists(Path::new(&processing_unit.input)) {
        panic!("{:?}", &processing_unit.input);
    }

    let mut rdr = Reader::from_path(&processing_unit.input).unwrap();
    let headers: Vec<String> = rdr.headers()
            .unwrap()
            .iter()
            .map(|s| String::from(s).replace("\'", "\\\""))
            .collect();
        
    write_to_file(rdr, &headers, &processing_unit.output)
}

pub fn write_to_file(mut rdr: Reader<File>, headers: &[String], output: &PathBuf) {
    if let Ok(mut file_handler) = File::create(output) {
        rdr.records().for_each(|optional_record| {
            if let Ok(record) = optional_record {
                let converted_line_output = convert_line(headers, &record);
                let _ = file_handler.write_all(converted_line_output.as_bytes());
            }
        });
    }
}
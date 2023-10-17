use reqwest::header;

// use std::{
//     borrow::Borrow,
//     error::Error,
//     fs::File,
//     io::{Write, Read},
//     path::{Path, PathBuf},
//     string::String,
// };

use std::fs::{File, self};
use std::path::PathBuf;
use std::string::String;
use std::io::{Read ,Write};

use csv::{Reader, StringRecord};
// use 

// implement csv management
//  read file 50 %
//  write file 20 %


//  sort file https://github.com/askingalot/rust-csv-file-sorter/blob/master/src/csv_sorter.rs

#[derive(Debug)]
pub struct Csv {
    header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Csv {
    pub fn new(mut rows: Vec<Vec<String>>) -> Self {
        let header = rows.remove(0)
            .iter()
            .map(|col| col.replace("\"", ""))
            .collect();

        Csv {
            header: header,
            rows: rows,
        }
    }

    pub fn sort_by(&mut self, col_name: String) {
        let col_index = self.header.iter().position(|h| h == &col_name);

        match col_index {
            Some(index) => self.rows.sort_by(|a, b| a[index].cmp(&(b[index]))),
            None => panic!("Could not find column: {}", col_name),
        }
    }

    pub fn filter(&mut self, query: &[String], rows: &Vec<Vec<String>>) -> Self {
        let mut result = Vec::new();
        for row in rows {
            if row.iter().any(|field|  query.contains(&field.to_string())) {
                let nrow = row.iter().map(|field| field.to_string()).collect::<Vec<String>>();
                result.push(nrow.clone())
            }
        } 
        let header =  rows[0].clone();
        Csv {
            header: header,
            rows: result,
        }
    }

}

pub fn get_csv_records(filename: &str) -> Vec<Vec<String>> {
    let mut csv_contents = String::new();
    let mut csv_file = File::open(filename).expect("Cannot open file!");
    csv_file.read_to_string(&mut csv_contents).expect("Cannot read file!");
    
    csv_contents.split('\n')
    .take_while(|&line| line.len() > 0)
    .map(|line| line.split(',').map(|col| col.to_string()).collect())
    .collect()
}



//  filter 0 % 
//  then publish into json file 10 %
// implement manage file
// jason https://rust-lang-nursery.github.io/rust-cookbook/encoding/complex.html
// jason -> csv https://github.com/dariusgm/CsvToJson/blob/main/src/lib.rs
// https://github.com/jbelmont/csv-to-json/blob/master/src/main.rs
// toml 

// jason https://github.com/serde-rs/json


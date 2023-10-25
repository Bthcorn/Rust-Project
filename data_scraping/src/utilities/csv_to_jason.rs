use std::{
    borrow::Borrow,
    error::Error,
    fs::File,
    io::{Write, Read},
    path::{Path, PathBuf},
    string::String,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
pub struct ProcessingUnit {
    input: PathBuf,
    output: PathBuf,
}

pub fn convert_line(headers: &[String], record: &Vec<Vec<String>>) -> String {
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
    a.push_str("},\n");
    nline
}

pub fn write_to_file(mut rdr: Reader<file>, headers: &[String], output: &PathBuf) {
    if let Ok(mut file_handler) = File::create(output) {
        file_handler.write("[\n".as_bytes());
        rdr.records().for_each(|optional_record| {
            if let Ok(record) = optional_record {
                let converted_line_output = convert_line(headers, &record);
                let _ = file_handler.write_all(converted_line_output.as_bytes());
            }
        });
        file_handler.write("\n]".as_bytes());
    }
}

fn build_output_path(output: &Option<String>, input: &Path ) -> PathBuf {
    let mut output_directory = match output {
        None => PathBuf::new(),
        Some(o) => {
            let x = PathBuf::from(o);
            if x.to_string_lossy().contains(".jason") {
                return x
            } else {
                x
            }
        }
    };

    let elements = input.iter();
    let size = input.iter().count();
    for (index, part) in elements.enumerate() {
        let casted_index = index as i32;
        let casted_size = size as i32 - 1;
        if casted_index < casted_size {
            output_directory.push(part);
        }
    }

    fs::create_dir_all(&output_directory).unwrap(); 
    let mut last = input.iter().last().unwrap().to_string();
    last.push(".json");
    output_directory.push(last);

    output_directory
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

// decide which one you gonna used 
// next make it can run and output file properly
// then write clap by yourself

// optional do for xml
// filter, word Freqency https://users.rust-lang.org/t/efficient-string-hashmaps-for-a-frequency-count/7752 
// https://exercism.org/tracks/rust/exercises/word-count/solutions/algorithm69
// https://exercism.org/tracks/rust/exercises/word-count/solutions/svetlanatanasov
// writing clap

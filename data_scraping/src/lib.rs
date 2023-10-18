// use reqwest::header;

use std::{
    borrow::Borrow,
    error::Error,
    fs::File,
    io::{Write, Read},
    path::{Path, PathBuf},
    string::String,
};
use clap::{App, Arg, SubCommand, ArgMatches};

use scraper::{Html, Selector};
use reqwest;

use csv::{Reader, StringRecord, Writer};

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
        for row in &self.rows {
            if row.iter().any(|field|  query.contains(&field.to_string())) {
                let nrow = row.iter().map(|field| field.to_string()).collect::<Vec<String>>();
                result.push(nrow.clone())
            }
        } 
        let header =  self.header.clone();
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

pub fn scraping(url: &str) -> Vec<Vec<String>> {
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let body = response.text().unwrap();

    let document = Html::parse_document(&body);
    let table_selector = Selector::parse("table").expect("Failed to create table selector");
    let row_selector = Selector::parse("tr").expect("Failed to create row selector");
    let head_selector = Selector::parse("th").expect("Failed to create headers row");
    let cell_selector = Selector::parse("td").expect("Failed to create cell selector");
    let main_table = document
        .select(&table_selector)
        .max_by_key(|table| table.select(&row_selector).count())
        .map(Option::Some)
        .unwrap_or(None);
    // Select and iterate through the table rows
    let mut rows: Vec<Vec<String>> = Vec::new();

    if let Some(table) = main_table {
        for row in table.select(&row_selector) {
            // Process and extract data from the table cells (td elements)
            let mut row_data: Vec<_> = Vec::new();
            for headers in row.select(&head_selector) {
                let cell_header = headers.text().collect::<String>().trim().to_string();
                row_data.push(cell_header);
            };

            for cell in row.select(&cell_selector) {
                let cell_text = cell.text().collect::<String>().trim().to_string();
                // let data = cell_text.to_string().clone();
                row_data.push(cell_text);
                // println!("Cell Text: {:?}", cell_text);
            }
            rows.push(row_data)
        }
    } else {
        println!("No matching table found.");
    }
   rows
}

pub fn write_to_csv(file_path: &str, rows: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            return Err(err.into());
        }
    };

    let mut wtr = Writer::from_writer(file);
    for row in rows {
        wtr.write_record(&row)?;
    }
    wtr.flush()?;
    Ok(())
}
#[derive(Debug)]
pub struct ConfigApp {
    subcommand: String,
    input: Option<String>,
    output: Option<String>,
    url: Option<String>,
    filter: Option<Vec<String>>,
    sort: Option<String>,
}

pub fn get_args() -> Result<(ConfigApp), Box<dyn Error>> {
    let matches = App::new("data_Scraping")
        .version("0.1.0")
        .author("Bowornthat SE15 <66010968@kmitl.ac.th>")
        .about("Table scraping")
        .subcommand(
            SubCommand::with_name("convert")
                .about("Convert a CSV file to a JASON file")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("CSV_FILE")
                        .help("Input CSV file")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("JSON_FILE")
                        .help("Output JASON file")
                        .default_value("default.json")
                        .required(true)
                        .takes_value(true)
                ),
        ).subcommand(
            SubCommand::with_name("scraping")
                .about("Scrap table data from URL and save it as CSV file")
                .arg(
                    Arg::with_name("url")
                        .short("u")
                        .long("url")
                        .value_name("URL")
                        .help("URL for scrap table data")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("CSV_FILE")
                        .help("Output CSV file")
                        .default_value("default.csv")
                        .takes_value(true)
                        .required(true)
                )
        ).subcommand(
            SubCommand::with_name("analyze")
                .about("Sort, filter columns in a CSV file")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("CSV_FILE")
                        .help("Input CSV file")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("CSV_FILE")
                        .help("Output CSV file")
                        .default_value("default.csv")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("filter")
                        .short("f")
                        .long("filter")
                        .value_name("FILTER_WORD")
                        .help("Filter keyword(s)")
                        .multiple(true)
                )
                .arg(
                    Arg::with_name("sort")
                        .short("s")
                        .long("sort")
                        .value_name("SORT_COL")
                        .help("Sort a column")
                        .takes_value(true)
                )
        )
        .get_matches();
    
    let config = match matches.subcommand() {
        ("convert", Some(convert_matches)) => {
            ConfigApp {
            subcommand: matches.subcommand().0.to_string(),
            input: convert_matches.value_of("input").map(|v| v.to_string()),
            output: convert_matches.value_of("output").map(|v| v.to_string()),
            url: None,
            filter: None,
            sort: None,
            }
            // println!("Converting CSV file: {} to JSON file: {}", input_file, output_file);
        }
        ("scraping", Some(scrap_matches)) => {
            ConfigApp {
                subcommand: matches.subcommand().0.to_string(),
                input: None,
                output: scrap_matches.value_of("output").map(|v| v.to_string()),
                url: scrap_matches.value_of("url").map(|v| v.to_string()),
                filter: None,
                sort: None,
                }
            // println!("Scraping url: {} to CSV file: {}", url, output_file);

        }
        ("analyze", Some(analyze_matches)) => { 
            let filter: Option<Vec<String>> = analyze_matches
                .values_of("filter")
                .map(|values| values.map(String::from).collect());
            ConfigApp {
                subcommand: matches.subcommand().0.to_string(),
                input: analyze_matches.value_of("input").map(|v| v.to_string()),
                output: analyze_matches.value_of("output").map(|v| v.to_string()),
                url: None,
                filter,
                sort: analyze_matches.value_of("sort").map(|v| v.to_string()),
                }
        }
        _ => {
            println!("Please specify a subcommand. Use --help for usage information.");
            std::process::exit(1);
        }
    };
    Ok(config)
}

pub fn run(config: ConfigApp) -> Result<(), Box<dyn Error>> {
    match config.subcommand.as_str() {
        "convert" => run_convert(&config),
        "scraping"=> run_scraping(&config),
        "analyze" => run_analyze(&config),
        _ => Ok(()),
    }
}

pub fn run_convert(config: &ConfigApp) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

pub fn run_scraping(config: &ConfigApp) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

pub fn run_analyze(config: &ConfigApp) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

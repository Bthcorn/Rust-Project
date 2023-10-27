use clap::{App, Arg, SubCommand};
use csv::{Reader, StringRecord, Writer};
use reqwest;
use scraper::{Html, Selector};
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    string::String,
};

// use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
pub struct Csv {
    header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Csv {
    pub fn new(mut rows: Vec<Vec<String>>) -> Self {
        let header = rows
            .remove(0)
            .iter()
            .map(|col| col.replace("\"", ""))
            .collect();

        Csv {
            header: header,
            rows: rows,
        }
    }
    // sort the column by choosing the index of the name column
    pub fn sort_by(&mut self, col_name: &str) {
        let col_index = self.header.iter().position(|h| h == &col_name);

        match col_index {
            Some(index) => self.rows.sort_by(|a, b| a[index].cmp(&(b[index]))),
            None => panic!("Could not find column: {}", col_name),
        }
    }

    pub fn filter(&mut self, query: &String) -> Self {
        let mut result = Vec::new();
        for row in &self.rows {
            if row.iter().any(|field| query == field) {
                result.push(row.clone())
            }
        }
        let header = self.header.clone();
        Csv {
            header: header,
            rows: result,
        }
    }

    pub fn write_csv(&mut self, file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = match File::create(file_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error creating file: {}", err);
                return Err(err.into());
            }
        };
        let mut wtr = Writer::from_writer(file);
        wtr.write_record(&self.header).expect("expect header");
        for row in &self.rows {
            wtr.write_record(row)?;
        }
        wtr.flush()?;
        Ok(())
    }

    pub fn get_column(&mut self, query: &String) -> Vec<String> {
        let id_col = self.header.iter().position(|h| h == query);
        let mut result = Vec::new();
        match id_col {
            Some(index) => {
                for row in &self.rows {
                    result.push(row[index].clone())
                }
            }
            None => panic!("Could not find column: {}", query),
        }
        result
    }
}

pub fn get_csv_records(filename: &PathBuf) -> Vec<Vec<String>> {
    let mut csv_contents = String::new();
    let mut csv_file = File::open(filename).expect("Cannot open file!");
    csv_file
        .read_to_string(&mut csv_contents)
        .expect("Cannot read file!");

    csv_contents
        .split('\n')
        .take_while(|&line| line.len() > 0)
        .map(|line| line.split(',').map(|col| col.to_string()).collect())
        .collect()
}

pub fn gen_svg_bar(names: &Vec<String>, values: &Vec<String>) -> String {
    let mut content = String::new();
    let nvalues = values
        .iter()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let max_value = nvalues.iter().max().unwrap_or(&0);

    // Define the dimensions of the SVG casnvas
    let width = 400;
    let height = 300;

    // Write the SVG header
    content.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
        width, height
    ));

    let bar_x_offset = width / nvalues.len() as u32;

    // Iterate over the data and create bars and labels
    for (i, (name, value)) in names.iter().zip(nvalues.iter()).enumerate() {
        let x = bar_x_offset * i as u32;
        let y = height - (value * height / max_value);

        // Create the bar
        content.push_str(&format!(
            r#"<Circle cx="{}" cy="{}" r="2" fill="red" />"#,
            x,
            y,
        ));

        // Create the label
        content.push_str(&format!(
            r#"<text x="{}" y="{}" text-anchor= "middle" font-size="5"  fill="black">{}</text>"#,
            x + bar_x_offset,
            y,
            name
        ))
    }
    content.push_str("</svg>");
    content
}
// ===========================================================

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
            }

            for cell in row.select(&cell_selector) {
                let cell_text = cell.text().collect::<String>().trim().to_string();
                row_data.push(cell_text);
            }
            rows.push(row_data)
        }
    } else {
        println!("No matching table found.");
    }
    rows
}

pub fn write_to_csv(file_path: &PathBuf, rows: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
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
// ===========================================================
#[derive(Debug)]
pub struct ProcessingUnit {
    input: PathBuf,
    output: PathBuf,
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
    let headers: Vec<String> = rdr
        .headers()
        .unwrap()
        .iter()
        .map(|s| String::from(s).replace("\'", "\\\""))
        .collect();

    write_to_json(rdr, &headers, &processing_unit.output)
}

pub fn write_to_json(mut rdr: Reader<File>, headers: &[String], output: &PathBuf) {
    if let Ok(mut file_handler) = File::create(output) {
        let mut object_started = false; // Track whether an object has started
        let _ = file_handler.write("[\n".as_bytes());

        rdr.records().for_each(|optional_record| {
            if let Ok(record) = optional_record {
                if object_started {
                    let _ = file_handler.write(",".as_bytes());
                }

                let converted_line_output = convert_line(headers, &record);
                let _ = file_handler.write_all(converted_line_output.as_bytes());

                object_started = true; // Set to true after the first record in an object
            }
        });

        let _ = file_handler.write("\n]".as_bytes());
    }
}
// ===========================================================
#[derive(Debug)]
pub struct ConfigApp {
    subcommand: String,
    input: Option<String>,
    output: Option<String>,
    url: Option<String>,
    filter: Option<String>,
    sort: Option<String>,
}

pub fn get_args() -> Result<ConfigApp, Box<dyn Error>> {
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
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("JSON_FILE")
                        .help("Output JASON file")
                        .default_value("default.json")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("scraping")
                .about("Scrap table data from URL and save it as CSV file")
                .arg(
                    Arg::with_name("url")
                        .short("u")
                        .long("url")
                        .value_name("URL")
                        .help("URL for scrap table data")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("CSV_FILE")
                        .help("Output CSV file")
                        .default_value("default.csv")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("analyze")
                .about("Sort, filter columns in a CSV file")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("CSV_FILE")
                        .help("Input CSV file")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("CSV_FILE")
                        .help("Output CSV file")
                        .default_value("default.csv")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("filter")
                        .short("f")
                        .long("filter")
                        .value_name("FILTER_WORD")
                        .help("Filter keyword(s)")
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("sort")
                        .short("s")
                        .long("sort")
                        .value_name("SORT_COL")
                        .help("Sort a column")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let config = match matches.subcommand() {
        ("convert", Some(convert_matches)) => ConfigApp {
            subcommand: matches.subcommand().0.to_string(),
            input: convert_matches.value_of("input").map(|v| v.to_string()),
            output: convert_matches.value_of("output").map(|v| v.to_string()),
            url: None,
            filter: None,
            sort: None,
        },
        ("scraping", Some(scrap_matches)) => ConfigApp {
            subcommand: matches.subcommand().0.to_string(),
            input: None,
            output: scrap_matches.value_of("output").map(|v| v.to_string()),
            url: scrap_matches.value_of("url").map(|v| v.to_string()),
            filter: None,
            sort: None,
        },
        ("analyze", Some(analyze_matches)) => {
            // let filter: Option<Vec<String>> = analyze_matches
            //     .values_of("filter")
            //     .map(|values| values.map(String::from).collect());
            ConfigApp {
                subcommand: matches.subcommand().0.to_string(),
                input: analyze_matches.value_of("input").map(|v| v.to_string()),
                output: analyze_matches.value_of("output").map(|v| v.to_string()),
                url: None,
                filter: analyze_matches
                    .values_of("filter")
                    .map(|values| values.map(String::from).collect()),
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
        "scraping" => run_scraping(&config),
        "analyze" => run_analyze(&config),
        _ => Ok(()),
    }
}

pub fn run_convert(config: &ConfigApp) -> Result<(), Box<dyn Error>> {
    let input = PathBuf::from(config.input.as_ref().unwrap());
    let output = PathBuf::from(config.output.as_ref().unwrap());
    let processing_unit = ProcessingUnit { input, output };
    convert_data(&processing_unit);
    Ok(())
}

pub fn run_scraping(config: &ConfigApp) -> Result<(), Box<dyn Error>> {
    let url = config.url.as_ref().unwrap();
    let rows = scraping(&url);
    let file_path = PathBuf::from(config.output.as_ref().unwrap());
    let _ = write_to_csv(&file_path, rows);
    Ok(())
}

pub fn run_analyze(config: &ConfigApp) -> Result<(), Box<dyn Error>> {
    let input = PathBuf::from(config.input.as_ref().unwrap());
    let output = PathBuf::from(config.output.as_ref().unwrap());
    let rows = get_csv_records(&input);
    let mut csv = Csv::new(rows);
    if let Some(sort) = config.sort.as_ref() {
        csv.sort_by(&sort);
        let _ = csv.write_csv(&output);
    };
    if let Some(query) = config.filter.as_ref() {
        csv = csv.filter(&query);
        let _ = csv.write_csv(&output);
    };
    Ok(())
}

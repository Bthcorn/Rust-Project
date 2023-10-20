// use std::{error::Error, arch::x86_64::_andn_u32};

// use scraper::{Html, Selector};
// use std::{
//     fs::File,
//     io::{Read, Write},
// use clap::{App, Arg, SubCommand, ArgMatches};
// };
// fn main() {
//     let url = "https://en.wikipedia.org/wiki/List_of_prime_ministers_of_the_United_Kingdom";
//     let response = reqwest::blocking::get(url).expect("Could not load url.");
//     let body = response.text().unwrap();
//     // println!{"{}", body}

//     let document = Html::parse_document(&body);
//     let table_selector = Selector::parse("table").expect("Failed to create table selector");
//     let row_selector = Selector::parse("tr").expect("Failed to create row selector");
//     let head_selector = Selector::parse("th").expect("Failed to create headers row");
//     let cell_selector = Selector::parse("td").expect("Failed to create cell selector");
//     let main_table = document
//         .select(&table_selector)
//         .max_by_key(|table| table.select(&row_selector).count())
//         .map(Option::Some)
//         .unwrap_or(None);
//     // Select and iterate through the table rows
//     let mut rows: Vec<Vec<String>> = Vec::new();

//     if let Some(table) = main_table {
//         for row in table.select(&row_selector) {
//             // Process and extract data from the table cells (td elements)
//             let mut row_data: Vec<_> = Vec::new();
//             for headers in row.select(&head_selector) {
//                 let cell_header = headers.text().collect::<String>().trim().to_string();
//                 row_data.push(cell_header);
//             };

//             for cell in row.select(&cell_selector) {
//                 let cell_text = cell.text().collect::<String>().trim().to_string();
//                 // let data = cell_text.to_string().clone();
//                 row_data.push(cell_text);
//                 // println!("Cell Text: {:?}", cell_text);
//             }
//             rows.push(row_data)
//         }
//     } else {
    //         println!("No matching table found.");
    //     }
    //     println!("{:?}", rows);
    // }
    // pub struct ConfigApp {
        //     input: 
        // }
        
fn main() {
    // let matches = data_scraping::get_args();
    // if let Err(e) = data_scraping::run(&matches) {
    //     eprintln!("{}", e);
    //     std::process::exit(1);
    // }
    if let Err(e) = data_scraping::get_args().and_then(data_scraping::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
        
// fn get_args() {
//     let matches = App::new("data_Scraping")
//         .version("0.1.0")
//         .author("Bowornthat SE15 <66010968@kmitl.ac.th>")
//         .about("Table scraping")
//         .subcommand(
//             SubCommand::with_name("convert")
//                 .about("Convert a CSV file to a JASON file")
//                 .arg(
//                     Arg::with_name("input")
//                         .short("i")
//                         .long("input")
//                         .value_name("CSV_FILE")
//                         .help("Input CSV file")
//                         .takes_value(true)
//                         .required(true)
//                 )
//                 .arg(
//                     Arg::with_name("output")
//                         .short("o")
//                         .long("output")
//                         .value_name("JSON_FILE")
//                         .help("Output JASON file")
//                         .default_value("default.json")
//                         .required(true)
//                         // .takes_value(true)
//                 ),
//         ).subcommand(
//             SubCommand::with_name("scraping")
//                 .about("Scrap table data from URL and save it as CSV file")
//                 .arg(
//                     Arg::with_name("url")
//                         .short("u")
//                         .long("url")
//                         .value_name("URL")
//                         .help("URL for scrap table data")
//                         .takes_value(true)
//                         .required(true)
//                 )
//                 .arg(
//                     Arg::with_name("output")
//                         .short("o")
//                         .long("output")
//                         .value_name("CSV_FILE")
//                         .help("Output CSV file")
//                         .default_value("default.csv")
//                         .takes_value(true)
//                         .required(true)
//                         // .default_value("output.csv")
//                 )
//         ).subcommand(
//             SubCommand::with_name("analyze")
//                 .about("Sort, filter columns in a CSV file")
//                 .arg(
//                     Arg::with_name("input")
//                         .short("i")
//                         .long("input")
//                         .value_name("CSV_FILE")
//                         .help("Input CSV file")
//                         .required(true)
//                         .takes_value(true)
//                 )
//                 .arg(
//                     Arg::with_name("output")
//                         .short("o")
//                         .long("output")
//                         .value_name("CSV_FILE")
//                         .help("Output CSV file")
//                         .default_value("default.csv")
//                         .required(true)
//                         .takes_value(true)
//                 )
//                 .arg(
//                     Arg::with_name("filter")
//                         .short("f")
//                         .long("filter")
//                         .value_name("FILTER_WORD")
//                         .help("Filter keyword")
//                         .multiple(true)
//                 )
//                 .arg(
//                     Arg::with_name("sort")
//                         .short("s")
//                         .long("sort")
//                         .value_name("SORT_COL")
//                         .help("Sort a column")
//                         .takes_value(true)
//                 )
//         )
//         .get_matches();
//     match matches.subcommand() {
//         ("convert", Some(convert_matches)) => {
//             let input_file = convert_matches.value_of("input").unwrap();
//             let output_file = convert_matches.value_of("output").unwrap();
//             println!("Converting CSV file: {} to JSON file: {}", input_file, output_file);
//         }
//         ("scraping", Some(scrap_matches)) => {
//             let url = scrap_matches.value_of("url").unwrap();
//             let output_file = scrap_matches.value_of("output").unwrap();
//             println!("Scraping url: {} to CSV file: {}", url, output_file);

//         }
//         ("analyze", Some(analze_matches)) => {
//             let input_file = analze_matches.value_of("input").unwrap();
//             let output_file = analze_matches.value_of("output").unwrap(); 
//             let filter = analze_matches.value_of("filter").unwrap();
//             let sort = analze_matches.value_of("sort").unwrap();
//         }
//         _ => {
//             println!("Please specify a subcommand. Use --help for usage information.");
//         }
//     }
// }

// fn main() {
//     let csv_contents = data_scraping::get_csv_records("organizations-100.csv");
//     let mut csv = data_scraping::Csv::new(csv_contents);
//     // println!("{:?}", csv);
//     csv.sort_by("Founded".to_string());
//     println!("{:?}", csv.rows)
// }

// fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
//     match matches.subcommand() {
//         ("convert", Some(m)) => run_convert(&m),
//         ("scraping", Some(m)) => run_scraping(&m),
//         ("analyze", Some(m)) => run_analyze(&m),
//         _ => Ok(()),
//     }
//     // Ok(())
// }

// fn run_convert(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
//     unimplemented!()
// }

// fn run_scraping(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
//     unimplemented!()
// }

// fn run_analyze(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
//     unimplemented!()
// }

 
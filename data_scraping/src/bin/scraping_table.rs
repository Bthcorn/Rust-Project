use std::error::Error;

use scraper::{Html, Selector};
use reqwest;

use csv::Writer;
use std::fs::File;

fn main() {
    let url = "https://en.wikipedia.org/wiki/List_of_cities_in_New_Zealand";
    let rows = scraping(&url);
    let file_path = "test_output.csv";
    let _ = write_to_csv(file_path, rows);
    // let response = reqwest::blocking::get(url).expect("Could not load url.");
    // let body = response.text().unwrap();
    // // println!{"{}", body}

    // let document = Html::parse_document(&body);
    // let table_selector = Selector::parse("table").expect("Failed to create table selector");
    // let row_selector = Selector::parse("tr").expect("Failed to create row selector");
    // let head_selector = Selector::parse("th").expect("Failed to create headers row");
    // let cell_selector = Selector::parse("td").expect("Failed to create cell selector");
    // let main_table = document
    //     .select(&table_selector)
    //     .max_by_key(|table| table.select(&row_selector).count())
    //     .map(Option::Some)
    //     .unwrap_or(None);
    // // Select and iterate through the table rows
    // let mut rows: Vec<Vec<String>> = Vec::new();

    // if let Some(table) = main_table {
    //     for row in table.select(&row_selector) {
    //         // Process and extract data from the table cells (td elements)
    //         let mut row_data: Vec<_> = Vec::new();
    //         for headers in row.select(&head_selector) {
    //             let cell_header = headers.text().collect::<String>().trim().to_string();
    //             row_data.push(cell_header);
    //         };

    //         for cell in row.select(&cell_selector) {
    //             let cell_text = cell.text().collect::<String>().trim().to_string();
    //             // let data = cell_text.to_string().clone();
    //             row_data.push(cell_text);
    //             // println!("Cell Text: {:?}", cell_text);
    //         }
    //         rows.push(row_data)
    //     }
    // } else {
    //     println!("No matching table found.");
    //     return;
    // }
    // println!("{:?}", rows);
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

// implement fn write to csv file
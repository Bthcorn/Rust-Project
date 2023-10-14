use scraper::{Html, Selector};
use std::{
    fs::File,
    io::{Read, Write},
};
fn main() {
    let url = "https://en.wikipedia.org/wiki/List_of_prime_ministers_of_the_United_Kingdom";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let body = response.text().unwrap();
    // println!{"{}", body}

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
    println!("{:?}", rows);
}


// fn main() {
//     let csv_contents = data_scraping::get_csv_records("organizations-100.csv");
//     let mut csv = data_scraping::Csv::new(csv_contents);
//     // println!("{:?}", csv);
//     csv.sort_by("Founded".to_string());
//     println!("{:?}", csv.rows)
// }

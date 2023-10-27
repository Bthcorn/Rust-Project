use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "organizations-100.csv";
    let column_name = "Type"; // Change this to your column name

    // Open and read the CSV file using the ReaderBuilder
    let file = std::fs::File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Collect the values from the specified column into a Vec<String>
    let mut column_values: Vec<String> = Vec::new();
    let mut column_names: Vec<String> = Vec::new();


    for result in rdr.records() {
        let record = result?;
        if let Some(value) = record.get(8) {
            column_values.push(value.to_string());
        }
        if let Some(name) = record.get(2) {
            column_names.push(name.to_string());
        }
    }

    let svg_contect = data_scraping::gen_svg_bar(&column_names, &column_values);
    println!("{}", svg_contect);



    // Now, you have your column values in column_values
    // for value in &column_values {
    //     println!("{}", value);
    // }

    Ok(())
}

// use std::fs::File;
// use std::io::Write;

// fn main() -> Result<(), std::io::Error> {
//     // Define your data points (x, y coordinates) along with their names.
//     let data = vec![
//         ("Point 1", (50, 50)),
//         ("Point 2", (100, 100)),
//         ("Point 3", (200, 200)),
//         ("Point 4", (300, 300)),
//     ];

//     // Define the dimensions of the SVG canvas.
//     let svg_width = 400;
//     let svg_height = 400;

//     // Create and open an SVG file for writing.
//     let mut file = File::create("scatter_plot.svg")?;

//     // Write the SVG header and canvas dimensions.
//     write!(file, r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#, svg_width, svg_height)?;

//     // Iterate over the data points and draw circles with labels for each point.
//     for (name, (x, y)) in &data {
//         // Draw the circle for the data point.
//         write!(file, r#"<circle cx="{}" cy="{}" r="5" fill="red" stroke="black" stroke-width="1" />"#, x, y)?;

//         // Draw the label for the data point.
//         write!(file, r#"<text x="{}" y="{}" font-family="Arial" font-size="12" fill="black">{}</text>"#, x + 10, y - 10, name)?;
//     }

//     // Close the SVG file.
//     write!(file, "</svg>")?;

//     println!("Scatter plot with labels saved to scatter_plot.svg");

//     Ok(())
// }

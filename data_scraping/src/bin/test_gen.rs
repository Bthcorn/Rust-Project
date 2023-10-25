// use data_scraping::Csv;
// use std::path::PathBuf;

// use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
fn main() { }
//     let file_path = PathBuf::from("data.csv");
//     let rows = data_scraping::get_csv_records(&file_path);
//     let mut csv = Csv::new(rows);
//     let query = vec!["name".to_string(), "age".to_string()];
//     let csv1 = csv.get_visual(&query);
//     let svg = generate_svg(csv1);
//     println!("{}", svg);
// } 

// pub fn get_csv_records(filename: &PathBuf) -> Vec<Vec<String>> {
//     let mut csv_contents = String::new();
//     let mut csv_file = File::open(filename).expect("Cannot open file!");
//     csv_file.read_to_string(&mut csv_contents).expect("Cannot read file!");
    
//     csv_contents.split('\n')
//     .take_while(|&line| line.len() > 0)
//     .map(|line| line.split(',').map(|col| col.to_string()).collect())
//     .collect()
// }

pub fn generate_svg(vis: (Vec<String>, Vec<f64>)) -> String {
    let mut content = String::new();
    let (names, values) = vis.clone();
    let max_value = values.clone().into_iter().reduce(f64::max).unwrap_or(0.0);

    // Define the dimensions of the SVG casnvas
    let width = 400.0;
    let height = 300.0;

    // Write the SVG header
    content.push_str(
        &format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" 
  "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
"#,width, height));


    // Define the attributes for the bars and labels
    let bar_width = 30;
    let label_x_offset = 15;
    let bar_x_offset = 50;

    // Iterate over the data and create bars and labels
    for (i, (name, value)) in names.iter().zip(values.iter()).enumerate() {
        let x = bar_x_offset + i as u32 * (bar_width + 10);
        let y = height - (value * height / max_value);

        // Create the bar
        content.push_str(
            &format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="blue" />"#,
            x, y, bar_width, value * height / max_value)
        );

        // Create the label
        content.push_str(
            &format!(r#"<text x="{}" y="{}" font-size="12" fill="black">{}</text>"#,
            x + label_x_offset,
            height - 10.,
            name)
        )
    }

    // Write the closing SVG tag
    content.push_str("</svg>");
    content
}

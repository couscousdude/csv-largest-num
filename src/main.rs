use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

// 100% ai generated

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_path = String::new();
    println!("Enter the path to the CSV file:");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input_path)?;
    let input_path = input_path.trim();

    let file = File::open(input_path)?;
    let mut rdr = Reader::from_reader(file);

    let mut max_value = None;
    let mut max_position = (0, 0);

    for (row_index, result) in rdr.records().enumerate() {
        let record = result?;
        for (col_index, field) in record.iter().enumerate() {
            let value: f64 = field.parse().unwrap_or(f64::MIN);
            if max_value.is_none() || value > max_value.unwrap() {
                max_value = Some(value);
                max_position = (row_index, col_index);
            }
        }
    }

    match max_value {
        Some(val) => println!(
            "Largest number: {}, located at row: {}, column: {}",
            val,
            max_position.0 + 1,
            max_position.1 + 1
        ),
        None => println!("Could not find a largest number in the file."),
    }

    Ok(())
}

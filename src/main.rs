use anyhow::{Context, Result};
use csv::ReaderBuilder;
use std::io::{self, Write};

fn main() -> Result<()> {
    let path = ask_for_csv_path()?;
    match find_largest_number_in_csv(&path) {
        Ok((max_value, location)) => {
            println!(
                "The largest number in the CSV is: {} at row {}, column {}",
                max_value, location.0, location.1
            );
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }
    Ok(())
}

fn ask_for_csv_path() -> Result<String> {
    print!("Please enter the path to the CSV file: ");
    io::stdout().flush().context("Could not flush stdout")?;
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .context("Failed to read line")?;
    Ok(path.trim().to_string())
}

fn find_largest_number_in_csv(path: &str) -> Result<(f64, (usize, usize))> {
    let mut reader = ReaderBuilder::new()
        .from_path(path)
        .with_context(|| format!("Failed to read CSV file from path: {}", path))?;

    let mut max_value = f64::MIN;
    let mut location = (0, 0);

    for (row_index, result) in reader.records().enumerate() {
        let record =
            result.with_context(|| format!("Failed to read record at row {}", row_index + 1))?;
        for (col_index, field) in record.iter().enumerate() {
            match field.parse::<f64>() {
                Ok(value) => {
                    if value > max_value {
                        max_value = value;
                        location = (row_index + 1, col_index + 1);
                    }
                }
                Err(_) => {
                    // Skip values that cannot be parsed as f64
                }
            }
        }
    }

    if max_value == f64::MIN {
        Err(anyhow::anyhow!("No numeric values found in the CSV file."))
    } else {
        Ok((max_value, location))
    }
}

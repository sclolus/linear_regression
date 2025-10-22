use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

#[derive(Parser)]
#[grammar = "src/grammars/weights.pest"]
struct WeightParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("weights").unwrap_or("0.0,0.0".to_string());

    let (theta0, theta1): (f64, f64) = match WeightParser::parse(Rule::file, &contents) {
        Ok(pairs) => {
            let mut theta0: f64 = 0.0;
            let mut theta1: f64 = 0.0;

            for pair in pairs {
                // println!("{:?}: {}", pair.as_rule(), pair.as_str());

                match pair.as_rule() {
                    Rule::theta0 => {
                        theta0 = pair
                            .as_str()
                            .parse::<f64>()
                            .or(Err("theta0 was not a float after all"))? // 
                    }
                    Rule::theta1 => {
                        theta1 = pair
                            .as_str()
                            .parse::<f64>()
                            .or(Err("theta1 was not a float after all"))?
                    }
                    _ => {}
                }
            }
            (theta0, theta1)
        }
        Err(e) => {
            eprintln!("Failed to parse weights: {}, using defaults", e);
			(0.0, 0.0)
        }
    };

    let mut input = String::new();

    print!("Please input mileage: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Error: read on stdin failed: {}", e))?;
    println!();
    let mileage: f64 = input
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("Error: input is not a number: '{}': {}", input.trim(), e))?;
    let estimated_price = theta0 + (theta1 * mileage);

    println!("Mileage: {}", mileage);
    println!("theta0: {}, theta1: {}", theta0, theta1);
    println!("Estimated price: {}", estimated_price);
    Ok(())
}

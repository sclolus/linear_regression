use std::fs;
use std::io::Read;
use std::process;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/predictor.pest"]
struct WeightParser;
	
fn main() -> Result<(), Box<dyn std::error::Error>>{
	let mut file: fs::File = match fs::File::open("weights") {
		Ok(file) => file,
		Err(e) => {
			eprintln!("Could not open weights file: {}", e);
			process::exit(1);
		}
	};
	
	let mut contents = String::with_capacity(11);

	file.read_to_string(&mut contents)?;

	let (theta0, theta1): (f64, f64) = match WeightParser::parse(Rule::file, &contents) {
		Ok(pairs) => {
			let mut theta0: f64 = 0.0;
			let mut theta1: f64 = 0.0;

			for pair in pairs {
				println!("{:?}: {}", pair.as_rule(), pair.as_str());

				match pair.as_rule() {
					Rule::theta0 => theta0 = pair.as_str().parse::<f64>().expect("Theta0 Was not a float after all"),
					Rule::theta1 => theta1 = pair.as_str().parse::<f64>().expect("Theta1 Was not a float after all"),
					_ => {}
				}
			}
			(theta0, theta1)
		},
		Err(e) => {
			eprintln!("Failed to parse weights: {}", e);
			process::exit(1)
		}
	};
	let mileage = 254000f64;
	let estimated_price = theta0 + (theta1 * mileage);
	println!("theta0: {}, theta1: {}", theta0, theta1);
	println!("Estimated price: {}", estimated_price);
	Ok(())
}

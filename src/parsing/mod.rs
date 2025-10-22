#[derive(Debug)]
pub struct Entry {
	pub price: f64,
	pub mileage: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Stats {
	pub mean: f64,

	#[allow(dead_code)]
	pub variance: f64,
	
	pub std: f64,
}



pub mod weights {
	use std::fs;
	use pest::Parser;
	use pest_derive::Parser;

	#[derive(Parser)]
	#[grammar = "src/grammars/weights.pest"]
	struct WeightParser;

	
	pub fn parse_weights_file() -> Result<(f64, f64), Box<dyn std::error::Error>> {
		let contents = fs::read_to_string("weights").unwrap_or("0.0,0.0".to_string());

		match WeightParser::parse(Rule::file, &contents) {
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
				Ok((theta0, theta1))
			}
			Err(e) => {
				eprintln!("Failed to parse weights: {}, using defaults", e);
				Ok((0.0, 0.0))
			}
		}
	}
}


pub mod data {
	use super::Entry;
	use pest::Parser;
	use pest_derive::Parser;

	#[derive(Parser)]
	#[grammar = "./src/grammars/data.pest"]
	pub struct DataParser;

	pub fn parse_data_file() -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
		let file_contents =
			std::fs::read_to_string("data.csv").or(Err("Failed to read data.csv file"))?;
		let pairs = DataParser::parse(Rule::csv_file, &file_contents)
			.map_err(|e| format!("Failed to parse data.csv file: {}", e))?;

		let mut entries: Vec<Entry> = Vec::with_capacity(128);
		for pair in pairs.clone() {
			// println!("-{:?}: {}", pair.as_rule(), pair.as_str());
			match pair.as_rule() {
				Rule::record => {
					let mut price = 0.0;
					let mut mileage = 0.0;

					for pair in pair.into_inner() {
						// println!("--{:?}: {}", pair.as_rule(), pair.as_str());
						match pair.as_rule() {
							Rule::mileage => {
								mileage = pair
									.as_str()
									.trim()
									.parse::<f64>()
									.map_err(|e| format!("Failed to parse mileage: {}", e))?
							}
							Rule::price => {
								price = pair
									.as_str()
									.trim()
									.parse::<f64>()
									.map_err(|e| format!("Failed to parse price: {}", e))?
							}
							_ => {}
						}
					}
					entries.push(Entry { price, mileage })
				}
				_ => {}
			}
		}
		Ok(entries)
	}
}

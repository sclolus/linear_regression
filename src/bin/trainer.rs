use pest::Parser;
use pest_derive::Parser;
use clap;

#[derive(Parser)]
#[grammar = "./src/grammars/data.pest"]
struct DataParser;

#[derive(Debug)]
struct Entry {
    price: f64,
    mileage: f64,
}


#[derive(clap::Parser)]
#[command(name = "trainer")]
#[command(about = "Trains the linear regression model on the data.csv file", long_about = None)]
struct Cli {
	#[arg(long)]
	plot_data_only: boolean,
	#[arg(short, long, default_value_t = 0.1)]
	learning_rate: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_contents =
        std::fs::read_to_string("data.csv").or(Err("Failed to read data.csv file"))?;
    let pairs = DataParser::parse(Rule::csv_file, &file_contents)
        .map_err(|e| format!("Failed to parse data.csv file: {}", e))?;

    let mut entries: Vec<Entry> = Vec::with_capacity(128);
    for pair in pairs.clone() {
        println!("-{:?}: {}", pair.as_rule(), pair.as_str());
        match pair.as_rule() {
            Rule::record => {
                let mut price = 0.0;
                let mut mileage = 0.0;

                for pair in pair.into_inner() {
					println!("--{:?}: {}", pair.as_rule(), pair.as_str());
                    match pair.as_rule() {
                        Rule::mileage => {
                            price = pair
                                .as_str()
                                .trim()
                                .parse::<f64>()
                                .map_err(|e| format!("Failed to parse mileage: {}", e))?
                        }
                        Rule::price => {
                            mileage = pair
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

	let learning_rate = 0.5;		// Make this configurable?

	let mut theta0 = 0.0;
	let mut theta1 = 0.0;
		

    Ok(())
}

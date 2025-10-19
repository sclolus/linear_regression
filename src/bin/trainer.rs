use pest::Parser;
use pest_derive::Parser;
use clap::Parser as ClapParser;

#[derive(Parser)]
#[grammar = "./src/grammars/data.pest"]
struct DataParser;

#[derive(Debug)]
struct Entry {
    price: f64,
    mileage: f64,
}


#[derive(ClapParser, Debug)]
#[command(name = "trainer")]
#[command(about = "Trains the linear regression model on the data.csv file", long_about = None)]
struct Cli {
	#[arg(long, default_value_t = false)]
	plot_data_only: bool,
	
	#[arg(short, long, default_value_t = false)]
	plot: bool,
	
	#[arg(short, long, default_value_t = 0.1)]
	learning_rate: f64,
}

struct PlotArgs<'a> {
	plot_data_only: bool,
	theta0: f64,
	theta1: f64,
	data_points: &'a Vec<Entry>,
}

use plotters::prelude::*;

fn generate_plot(args: PlotArgs) -> Result<(), Box<dyn std::error::Error>> {
	let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

	let min_price = args.data_points.iter().map(|x| x.price).fold(f64::INFINITY, f64::min);
	let max_price = args.data_points.iter().map(|x| x.price).fold(f64::NEG_INFINITY, f64::max);

	let min_mileage = args.data_points.iter().map(|x| x.mileage).fold(f64::INFINITY, f64::min);
	let max_mileage = args.data_points.iter().map(|x| x.mileage).fold(f64::NEG_INFINITY, f64::max);
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Mileage-price linear regression ", ("sans-serif", 50))
        .margin(5)
        .x_label_area_size(64)
        .y_label_area_size(64)
        .build_cartesian_2d(min_mileage..max_mileage, min_price..max_price)?;
    
    chart.configure_mesh().draw()?;
    
    chart.draw_series(PointSeries::of_element(
        args.data_points.iter().map(|point| (point.mileage, point.price)),
		5,
        &RED,
		&|coord, size, style| {
			EmptyElement::at(coord) + Cross::new((0, 0), size, style)
		}
    ))?;

	if !args.plot_data_only {
		let theta0 = args.theta0;
		let theta1 = args.theta1;
			
		chart.draw_series(LineSeries::new(
			vec![(min_mileage, theta0 + theta1 * min_mileage),
			(max_mileage, theta0 + theta1 * max_mileage)],
			&GREEN,
		))?;
	}
    
    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Cli::parse();
	
	
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

	let learning_rate = args.learning_rate;		// Make this configurable?

	let mut theta0 = 10000.0;
	let mut theta1 = -0.1;
		

	println!("{:?}", args);
	println!("{:?}", entries);
	// let plot_data_only: bool = args.plot_data_only;
	let plot_data_only: bool = true;
	let plot: bool = args.plot;
		
	if plot_data_only || plot {
		generate_plot(PlotArgs {
			plot_data_only,
			theta0,
			theta1,
			data_points: &entries,
		})?
	}
    Ok(())
}

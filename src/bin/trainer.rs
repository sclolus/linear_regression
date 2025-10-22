use std::io::Write;

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

#[derive(Debug, Clone, Copy)]
struct Stats {
	mean: f64,

	#[allow(dead_code)]
	variance: f64,
	
	std: f64,
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

	#[arg(short, long, default_value_t = 10000)]
	iterations: usize,

	#[arg(short, long, default_value = None)]
	epsilon: Option<f64>,
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

		// println!("{:?}",	vec![(min_mileage, theta0 + theta1 * min_mileage),
		// 	(max_mileage, theta0 + theta1 * max_mileage)]);
		chart.draw_series(LineSeries::new(
			vec![(min_mileage, theta0 + theta1 * min_mileage),
			(max_mileage, theta0 + theta1 * max_mileage)],
			&GREEN,
		))?;
	}
    
    root.present()?;
    Ok(())
}

fn normalize_entries(entries: &Vec<Entry>) -> (Vec<Entry>, Stats, Stats) {
	let n: f64 = entries.len() as f64;
	let mean_price: f64 = entries.iter().map(|e| e.price).sum::<f64>() / n;
	let mean_mileage: f64 = entries.iter().map(|e| e.mileage).sum::<f64>() / n;

	let price_variance: f64 = entries.iter().map(|e| (mean_price - e.price).powi(2)).sum::<f64>() / n;
	let mileage_variance: f64 = entries.iter().map(|e| (mean_mileage - e.mileage).powi(2)).sum::<f64>() / n;

	let price_std: f64 = price_variance.sqrt();
	let mileage_std: f64 = mileage_variance.sqrt();
		

	let price_stats = Stats {
		mean: mean_price,
		variance: price_variance,
		std: price_std,
	};
	
	let mileage_stats = Stats {
		mean: mean_mileage,
		variance: mileage_variance,
		std: mileage_std,
	};

	let normalized_entries = entries.iter().map(|e| Entry {
		mileage: (e.mileage - mean_mileage) / mileage_std,
		price: (e.price - mean_price) / price_std
	}).collect();

	(normalized_entries, mileage_stats, price_stats)
}

fn denormalize_model_parameters(beta0: f64, beta1: f64, mileage_stats: Stats, price_stats: Stats) -> (f64, f64) {
	// Those formula follow from price = theta0 + theta1 * mileage AND from the normalized model: (price - mean_price) / std_price = beta0 + beta1 * ((mileage - mean_mileage) / std_mileage).
	let theta0: f64 = beta0 * price_stats.std + price_stats.mean - beta1 * (price_stats.std / mileage_stats.std) * mileage_stats.mean;
	let theta1: f64 = beta1 * price_stats.std / mileage_stats.std;

	(theta0, theta1)
}

fn linear_regression(entries: &Vec<Entry>, learning_rate: f64, iterations: usize, epsilon: Option<f64>) -> (f64, f64) {
	let mut theta0 = 0.0;
	let mut theta1 = 0.0;

	let m: f64 = entries.len() as f64;

	let mut i: usize = 0;
	
	while i < iterations {
		let residuals_sum: f64 = entries.iter().map(|e| (theta0 + theta1 * e.mileage) - e.price).sum();
		let residuals_times_theta1_sum: f64 = entries.iter().map(|e| ((theta0 + theta1 * e.mileage) - e.price) * e.mileage).sum();

		theta0 -= learning_rate * (1.0 / m) * residuals_sum;
		theta1 -= learning_rate * (1.0 / m) * residuals_times_theta1_sum;
		// theta0 = learning_rate * (1.0 / m) * residuals_sum;
		// theta1 = learning_rate * (1.0 / m) * residuals_times_theta1_sum;

		// let cost = 1.0 / (2.0 * (m as f64)) * entries.iter().map(|e| ((theta0 + theta1 * e.mileage) - e.price).powi(2)).sum::<f64>();
		// println!("Regressed #{} to (theta0, theta1, cost): ({:32}, {:32}, {:32})", i, theta0, theta1, cost);
		i += 1 ;
	}

	(theta0, theta1)
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

	let learning_rate = args.learning_rate;
	let iterations = args.iterations;
	// println!("{:?}", entries);
	// println!("{:?}", args);

	let (normalized_entries, mileage_stats, price_stats): (Vec<Entry>, Stats, Stats) = normalize_entries(&entries);
	// println!("{:?}", normalized_entries);
	let (beta0, beta1) = linear_regression(&normalized_entries, learning_rate, iterations, None);
	let (theta0, theta1) = denormalize_model_parameters(beta0, beta1, mileage_stats, price_stats);
	println!("Regressed to (theta0, theta1): ({}, {}) with {} iterations and learning rate = {}", theta0, theta1, iterations, learning_rate);

	let weights_file_content = format!("{},{}", theta0, theta1);
	let mut file = std::fs::OpenOptions::new().create(true).write(true).open("weights")?;
	file.write_all(weights_file_content.as_bytes())?;
	
	let plot_data_only: bool = args.plot_data_only;
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

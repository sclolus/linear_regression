use std::io::Write;

use ft_linear_regression::parsing::{Entry, Stats};
use ft_linear_regression::parsing::data::parse_data_file;

use clap::Parser as ClapParser;

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

	#[arg(short, long, default_value_t = 1000)]
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
	let (normalized_entries, mileage_stats, price_stats): (Vec<Entry>, Stats, Stats) = normalize_entries(&entries);
	// println!("{:?}", normalized_entries);

	let mut theta0 = 0.0;
	let mut theta1 = 0.0;

	let m: f64 = normalized_entries.len() as f64;

	let mut i: usize = 0;
	let mut old_cost: Option<f64> = None;
	let mut cost: f64 = 0.0;
	
	while i < iterations {
		let residuals_sum: f64 = normalized_entries.iter().map(|e| (theta0 + theta1 * e.mileage) - e.price).sum();
		let residuals_times_theta1_sum: f64 = normalized_entries.iter().map(|e| ((theta0 + theta1 * e.mileage) - e.price) * e.mileage).sum();

		theta0 -= learning_rate * (1.0 / m) * residuals_sum;
		theta1 -= learning_rate * (1.0 / m) * residuals_times_theta1_sum;
		// theta0 = learning_rate * (1.0 / m) * residuals_sum;
		// theta1 = learning_rate * (1.0 / m) * residuals_times_theta1_sum;

		cost = 1.0 / (2.0 * (m as f64)) * normalized_entries.iter().map(|e| ((theta0 + theta1 * e.mileage) - e.price).powi(2)).sum::<f64>();
		// println!("Regressed #{} to (theta0, theta1, cost): ({:32}, {:32}, {:32})", i, theta0, theta1, cost);

		if let Some(epsilon) = epsilon && let Some(old_cost) = old_cost && (old_cost - cost).abs() < epsilon {
			break;
		}
		old_cost = Some(cost);

		i += 1 ;
	}

	let (denorm_theta0, denorm_theta1) = denormalize_model_parameters(theta0, theta1, mileage_stats, price_stats);
	println!("Regressed to (theta0, theta1): ({}, {}) with {} iterations, learning rate = {} and epsilon = {:?}", denorm_theta0, denorm_theta1, i, learning_rate, epsilon);
	println!("Final cost: {}", cost);

	(denorm_theta0, denorm_theta1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Cli::parse();

	let entries = parse_data_file()?;

	let learning_rate = args.learning_rate;
	let iterations = args.iterations;
	let epsilon = args.epsilon;
	// println!("{:?}", entries);
	// println!("{:?}", args);

	let (theta0, theta1) = linear_regression(&entries, learning_rate, iterations, epsilon);

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

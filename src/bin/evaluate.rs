use ft_linear_regression::parsing::{Entry};
use ft_linear_regression::parsing::data::parse_data_file;
use ft_linear_regression::parsing::weights::parse_weights_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let entries: Vec<Entry> = parse_data_file()?;
	let (theta0, theta1): (f64, f64) = parse_weights_file()?;

	let n: f64 = entries.len() as f64;
	let price_mean: f64 = entries.iter().map(|e| e.price).sum::<f64>() / n;
	let residuals_squared_sum: f64 = entries.iter().map(|e| (e.price - (theta0 + theta1 * e.mileage)).powi(2)).sum();
	let total_squared_sum = entries.iter().map(|e| (e.price - price_mean).powi(2)).sum::<f64>();
	let r_squared: f64 = 1.0 - (residuals_squared_sum / total_squared_sum);

	let indicator = match r_squared {
		0.0..0.20 => "very bad",
		0.20..0.40 => "bad",
		0.40..0.60 => "moderate",
		0.60..0.75 => "good",
		0.75..0.90 => "very good",
		0.90..1.0 => "excellent",
		_ => "Should not happen."
	};
	
	println!("The model explains {:.2}% of the car's price's variance.", r_squared * 100.0);
	println!("R^2 (R squared): {:.2} ({})", r_squared, indicator);
    Ok(())
}

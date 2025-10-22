use std::io;
use std::io::Write;
use ft_linear_regression::parsing::weights::parse_weights_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let (theta0, theta1): (f64, f64) = parse_weights_file()?;
	
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

use clap::Parser; // Import the Parser trait from clap

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Calculate sum and mean of two numbers")]
struct Args {
    num1: i64,
    num2: i64,
}

// Function to calculate the sum of two numbers
fn calculate_sum(x: i64, y: i64) -> i64 {
    x + y
}

// Function to calculate the mean of two numbers
fn calculate_mean(x: i64, y: i64) -> f64 {
    (x + y) as f64 / 2.0
}

fn main() {
    let args = Args::parse(); // Parse the command-line arguments

    let sum = calculate_sum(args.num1, args.num2);
    let mean = calculate_mean(args.num1, args.num2);
    
    println!("The sum of num1 and num2: {}", sum);
    println!("The mean of num1 and num2: {}", mean);
}

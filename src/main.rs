use plotters::prelude::*;
use rand::Rng;
use std::io;

fn func(x: f64) -> f64 {
    x * x.sin()
}

fn monte_carlo(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> (f64, f64) {
    let mut rng = rand::rng();
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    let inter = b - a;
    let f_vals = (0..n)
        .map(|_| rng.random::<f64>() * inter + a)
        .map(f)
        .collect::<Vec<_>>();
    let f_mean = f_vals.iter().sum::<f64>() / n as f64;
    let f_mean_sq = f_vals.iter().map(|x| x.powi(2)).sum::<f64>() / n as f64;
    (
        inter * f_mean,
        (f_mean_sq - f_mean.powi(2)).sqrt() / (n as f64).sqrt(),
    )
}

fn minmax(v: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    v.iter().copied().fold(
        (
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ),
        |(x_min, x_max, y_min, y_max), (x, y)| {
            (x_min.min(x), x_max.max(x), y_min.min(y), y_max.max(y))
        },
    )
}

fn plot(vals: Vec<(f64, f64)>) {
    let (x_min, x_max, y_min, y_max) = minmax(&vals);
    let root = BitMapBackend::new("plotters_plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Integration errors", ("sans-serif", 20).into_font())
        .x_label_area_size(50)
        .margin_right(30)
        .y_label_area_size(50)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();
    chart
        .configure_mesh()
        .x_max_light_lines(0)
        .y_max_light_lines(0)
        .x_desc("Error value")
        .y_desc("Number of MC steps")
        .draw()
        .unwrap();
    chart.draw_series(LineSeries::new(vals, &RED)).unwrap();
    root.present().unwrap();
    println!("Plot saved to plotters_plot.png");
}

fn main() {
    println!(
        "This program numerically (using Monte Carlo method) integrates sin(x) \nin the user specified interval (a, b)!"
    );
    println!("Please enter a:");
    let mut a_str = String::new();
    io::stdin()
        .read_line(&mut a_str)
        .expect("Failed to read line");
    let a: f64 = a_str.trim().parse().expect("Your number is not valid!");
    println!("Please enter b:");
    let mut b_str = String::new();
    io::stdin()
        .read_line(&mut b_str)
        .expect("Failed to read line");
    let b: f64 = b_str.trim().parse().expect("Your number is not valid!");
    let vals = (1..100)
        .map(|x| 10 * x)
        .map(|n| {
            let (int_val, err) = monte_carlo(func, a, b, n);
            println!("Integral result for n = {n} is {int_val} with err: {err}");
            (n as f64, err)
        })
        .collect::<Vec<_>>();
    let res = monte_carlo(func, a, b, vals[vals.len() - 1].0 as usize);
    println!("The final result is {} ± {}", res.0, res.1);
    plot(vals);

    println!(
        "Plot of calculation error versus the number of MC steps is saved. \nPress Enter to quit the application."
    );
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

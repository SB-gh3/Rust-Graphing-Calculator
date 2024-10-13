use std::io;
use gnuplot::{AxesCommon, Caption, Coordinate::Graph, Figure};

fn create_graph(x : Vec<f32>, y : Vec<f32>) -> Figure
{
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("A plot", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("x", &[])
        .set_y_label("y", &[])
        .lines(
            &x,
            &y,
            &[Caption("Parabola")],
        );

    return fg;
}

fn parabolic(x : Vec<f32>, mut y : Vec<f32>) -> Vec<f32>
{
    for i in x.into_iter()
    {
        y.push(i.powi(2));
    }

    return y;
}

fn sqrt(x : Vec<f32>, mut y : Vec<f32>) -> Vec<f32>
{
    for i in x.into_iter()
    {
        if !i.sqrt().is_nan()
        {
            y.push(i.sqrt());
        }
        else
        {
            y.push(0.);
        }
    }

    return y;
}

fn exponential(x : Vec<f32>, mut y : Vec<f32>) -> Vec<f32>
{
    for i in x.into_iter()
    {
        y.push((2 as f32).powi(i as i32));
    }

    return y;
}

fn reciprocal(x : Vec<f32>, mut y : Vec<f32>) -> Vec<f32>
{
    for i in x.into_iter()
    {
        y.push(1./i);
    }

    return y;
}

fn main() 
{
    let mut x : Vec<f32> = Vec::new();
    let mut input = String::new();
    let mut y : Vec<f32> = Vec::new();
    let mut fg : Figure = Figure::new();

    println!("Enter min-x value: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let min_x : i32 = input.trim().parse().expect("Invalid input");
    input.clear();

    println!("Enter max-x value: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let max_x : i32 = input.trim().parse().expect("Invalid input");
    input.clear();

    for n in min_x..(max_x + 1)
    {
        x.push(n as f32);
    }

    println!("Parent Function (parabolic, sqrt, exponential, reciprocal): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let choice : String = input.trim().parse().expect("Invalid input");
    input.clear();

    if choice == "parabolic"
    {
        y = parabolic(x.clone(), y.clone());
    }
    else if choice == "sqrt"
    {
        y = sqrt(x.clone(), y.clone());
    }
    else if choice == "exponential"
    {
        y = exponential(x.clone(), y.clone());
    }
    else if choice == "reciprocal"
    {
        y = reciprocal(x.clone(), y.clone());
    }

    fg = create_graph(x, y);

    fg.show().unwrap();
}
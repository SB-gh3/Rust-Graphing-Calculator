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
            &[Caption("Graph")],
        );

    return fg;
}

fn calculate(eq_top : Vec<f32>, eq_bottom : Vec<f32>, x : Vec<f32>, mut y : Vec<f32>) -> Vec<f32>
{
    let mut eq : f32 = 0.;

    if eq_bottom.is_empty()
    {
        for (_i, elemi) in x.clone().into_iter().enumerate()
        {
            for (j, elemj) in eq_top.clone().into_iter().enumerate()
            {
                eq += elemi.powi(j.try_into().unwrap())*elemj;
            }
            y.push(eq);
        }
    }
    else
    {

    }

    return y;
}

fn main() 
{
    let mut input = String::new();
    let mut x : Vec<f32> = Vec::new();
    let mut y : Vec<f32> = Vec::new();
    let mut fg : Figure = Figure::new();
    let mut eq_top : Vec<f32> = Vec::new();
    let mut eq_bottom : Vec<f32> = Vec::new();
    let mut boo : bool = false;

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

    loop
    {
        println!("Enter coefficient as decimal (/ to enter reciprocal, q to exit): ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let ch : String = input.trim().parse().expect("Invalid input");
        input.clear();

        if !ch.eq("/") && !ch.eq("q")
        {
            eq_top.push(ch.parse::<f32>().unwrap());
        }
        else if ch.eq("/")
        {
            boo = true;
            break;
        }
        else
        {
            break;
        }
    }

    if boo
    {
        loop
        {
            println!("Enter bottom coefficient as decimal (space to exit): ");
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let ch : String = input.trim().parse().expect("Invalid input");
            input.clear();

            if ch != " "
            {
                eq_bottom.push(ch.parse::<f32>().unwrap());
            }
            else
            {
                break;
            }
        }
    }

    eq_top.reverse();

    y = calculate(eq_top, eq_bottom, x.clone(), y.clone());

    fg = create_graph(x, y);

    fg.show().unwrap();
}
use std::io;
use gnuplot::{AutoOption::Fix, AxesCommon, Caption, Coordinate::Graph, Figure, PlotOption::{Color, LineWidth}};

fn display_graph(x : Vec<f32>, y : Vec<f32>)
{
    let mut fg : Figure = Figure::new();
    fg.axes2d()
        .set_title("Graph", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("x", &[])
        .set_y_label("y", &[])
        .set_y_range(Fix(-1.), Fix(1.))
        .set_x_range(Fix(-1.), Fix(1.))
        .lines(
            &x,
            &y,
            &[Caption("Graph"), LineWidth(3.), Color("black")],
        );

    fg.show().unwrap();
}

fn calculate(eq_top : &Vec<f32>, eq_bottom : &Vec<f32>, x : &Vec<f32>, mut y : Vec<f32>, ty : u8) -> Vec<f32>
{
    let mut eq_t : f32 = 0.;
    let mut eq_b : f32 = 0.;

    if eq_bottom.is_empty()
    {
        for (_i, elemi) in x.into_iter().enumerate()
        {
            for (j, elemj) in eq_top.into_iter().enumerate()
            {
                match ty
                {
                    1_u8 => eq_t += elemi.powf(j as f32) * elemj,
                    2_u8 => eq_t += elemi.powf(j as f32) * elemj,
                    3_u8 => eq_t += elemj.powf(j as f32 * elemi),
                    _ => println!("ERROR"),
                }
            }

            match ty
            {
                1_u8 => y.push(eq_t),
                2_u8 => y.push(eq_t.sqrt()),
                3_u8 => y.push(eq_t),
                _ => println!("ERROR"),
            }

            eq_t = 0.;
        }
    }
    else
    {
        for (_i, elemi) in x.into_iter().enumerate()
        {
            for (j, elemj) in eq_top.into_iter().enumerate()
            {
                match ty
                {
                    1_u8 => eq_t += elemi.powf(j as f32) * elemj,
                    2_u8 => eq_t += elemi.powf(j as f32) * elemj,
                    3_u8 => eq_t += elemj.powf(j as f32 * elemi),
                    _ => println!("ERROR"),
                }
            }

            for (k, elemk) in eq_bottom.into_iter().enumerate()
            {
                match ty
                {
                    1_u8 => eq_b += elemi.powf(k as f32) * elemk,
                    2_u8 => eq_b += elemi.powf(k as f32) * elemk,
                    3_u8 => eq_b += elemk.powf(k as f32 * elemi),
                    _ => println!("ERROR"),
                }
            }

            match ty
            {
                1_u8 => y.push(eq_t / eq_b),
                2_u8 => y.push((eq_t / eq_b).sqrt()),
                3_u8 => y.push(eq_t / eq_b),
                _ => println!("ERROR"),
            }

            eq_t = 0.;
            eq_b = 0.;
        }
    }

    return y;
}

fn main() 
{
    let mut input = String::new();
    let mut x : Vec<f32> = Vec::new();
    let mut y : Vec<f32> = Vec::new();
    let mut eq_top : Vec<f32> = Vec::new();
    let mut eq_bottom : Vec<f32> = Vec::new();
    let mut boo : bool = false;

    println!("1. Polynomial\n2. Square Root\n3. Exponential (Experimental)");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ty : u8 = input.trim().parse().expect("Invalid input");
    input.clear();

    println!("Enter min-x value: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let min_x : f32 = input.trim().parse().expect("Invalid input");
    input.clear();

    println!("Enter max-x value: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let max_x : f32 = input.trim().parse().expect("Invalid input");
    input.clear();

    for n in (min_x * 1000.) as i32..(max_x * 1000.) as i32
    {
        x.push(n as f32 / 1000.);
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
            println!("Enter bottom coefficient as decimal (q to exit): ");
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let ch : String = input.trim().parse().expect("Invalid input");
            input.clear();

            if !ch.eq("q")
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
    eq_bottom.reverse();

    y = calculate(&eq_top, &eq_bottom, &x, y, ty);

    display_graph(x, y);
}
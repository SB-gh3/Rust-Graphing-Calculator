use gnuplot::{AxesCommon, Caption, Coordinate::Graph, Figure};

fn display_graph(x : Vec<f32>, y : Vec<f32>) -> Figure
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

fn main() 
{
    let x : Vec<f32> = vec![-3., -2., -1., 0., 1., 2., 3.];
    let mut y : Vec<f32> = Vec::new();
    let mut fg : Figure = Figure::new();

    for i in x.clone().into_iter()
    {
        y.push(i.powi(2))
    }

    fg = display_graph(x, y);

    fg.show().unwrap();
}
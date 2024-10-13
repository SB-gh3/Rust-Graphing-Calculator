use gnuplot::{Figure, Caption, Color};


fn main() 
{
    let x = [0i32, 0, 0];
    let y = [3i32, -400, 500];
    let z = [6i32, -400, 500];
    let mut fg = Figure::new();
    fg.axes3d()
    .lines(&x, &y, &z, &[Caption("A line"), Color("black")]);
    let _ = fg.show();
}
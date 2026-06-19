// Plot Mandlebrot Set
// 19/06/2026
use plotters::prelude::*;
use num_complex::Complex;
use std::io;

fn escape_iter(c: Complex<f64>, max_iter: i32) -> i32{ // calculate escape time 
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..max_iter {
        z = z*z + c;
        if z.norm() >= 2.0{
            return i; // return number of iterations to diverge
        }
    }
    return max_iter;
}

fn escape_time_to_color(t: i32, max_iter: i32) -> RGBColor {
    if t >= max_iter {
        // Points that never escaped (inside the set) -> black
        return RGBColor(0, 0, 0);
    }

    // Normalize to [0, 1]
    let ratio = t as f64 / max_iter as f64;

    // Use ratio to drive hue around the color wheel
    let hue = (200.0 + 360.0 * ratio) % 360.0;
    hsv_to_rgb(hue, 1.0, 1.0)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> RGBColor {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = match h as i32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    RGBColor(
        ((r1 + m) * 255.0) as u8,
        ((g1 + m) * 255.0) as u8,
        ((b1 + m) * 255.0) as u8,
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let width = 800;
    let height = 800;
    let max_iter = 500;
    let root = BitMapBackend::new("mandelbrot.png", (width, height))
        .into_drawing_area();
    root.fill(&BLACK)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(0)
        .build_cartesian_2d(-2.0..2.0, -2.0..2.0)?;
    chart.configure_mesh().disable_mesh().draw()?;
    let plotting_area = chart.plotting_area();


    println!("What step size? (lower = higher resolution)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let step_size: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                return Err("Invalid floating-point input provided.".into());
            }
        };
    let bound = (2.0 / step_size).round() as i32;

    for i in -bound..bound { // iterate accross each pixel
        let x = i as f64 * step_size;
        for j in -bound..bound {
            let y = j as f64 * step_size;
            let c = Complex::new(x, y);
            let escape_time;
            if c.norm() < 2.0{
                escape_time = escape_iter(c, max_iter);
            }
            else{
                escape_time = max_iter;
            }
            let colour = escape_time_to_color(escape_time, max_iter);
            plotting_area.draw_pixel((x, y), &colour)?; // plot pixel based on escape time
        }
    }
    root.present()?;
    Ok(())


}
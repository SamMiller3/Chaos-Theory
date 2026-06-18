use plotters::prelude::*;
use std::io;

// 18/06/2026 Chaotic Lorenz attractors 

const OUT_FILE_NAME: &str = "3d-plot.svg";

fn main() {
    loop {
        println!("How many timesteps?: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let steps: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                return;
            }
        };
        graph(steps);

    }
}


fn graph(steps: i32) -> Result<(), Box<dyn std::error::Error>> {
    // initialise constants
    let beta: f64 = 8.0 / 3.0;
    let sigma: f64 = 10.0;
    let rho: f64 = 28.0;

    // start at (x,y,z) = (1.0,1.0,1.0)
     let mut x: f64 = 1.0;
     let mut y: f64 = 1.0;
     let mut z: f64 = 1.0; 
     let mut x_dot: f64;
     let mut y_dot: f64;
     let mut z_dot: f64;

     let mut points: Vec<(f64, f64, f64)> = vec![
        (1.0, 1.0, 1.0),
    ]; // store solutions to buffer to plot later


     // approximate solutions with eulers method
     let step: f64 = 0.001;
     let mut counter = 0;
     loop {
        x_dot = sigma * (y-x);
        y_dot = x * (rho - z) - y;
        z_dot = x * y - beta * z;
        x += step * x_dot;
        y += step * y_dot;
        z += step * z_dot;
        points.push((x, y, z));
        if counter == steps {
            break
        }
        counter+=1;
     }


    // plot using plotters crate
    let area = SVGBackend::new(OUT_FILE_NAME, (1024, 760)).into_drawing_area();
    area.fill(&WHITE)?;


    let (x_min, x_max) = points.iter().map(|p| p.0).fold((f64::MAX, f64::MIN), |(lo, hi), v| (lo.min(v), hi.max(v)));
    let (y_min, y_max) = points.iter().map(|p| p.1).fold((f64::MAX, f64::MIN), |(lo, hi), v| (lo.min(v), hi.max(v)));
    let (z_min, z_max) = points.iter().map(|p| p.2).fold((f64::MAX, f64::MIN), |(lo, hi), v| (lo.min(v), hi.max(v)));

    let mut chart = ChartBuilder::on(&area)
        .caption("3D Scatter Plot", ("sans", 20))
        .build_cartesian_3d(x_min..x_max, y_min..y_max, z_min..z_max)?;

    chart.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 0.9;
        pb.into_matrix()
    }); // adjust yaw (or pitch) to rotate

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    // as discrete points
    chart
        .draw_series(PointSeries::of_element(
            points.clone(),
            5,
            &RED,
            &|c, s, st| {
                EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
            },
        ))?
        .label("Points")
        .legend(|(x, y)| Circle::new((x + 7, y), 5, RED.filled()));

    chart.configure_series_labels().border_style(BLACK).draw()?;

    area.present()?;
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}
use std::{env, process};

use plotters::prelude::*;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let points = get_points(
        format!("images/{}", config.filename).as_str(),
        config.color_channel,
    );
    plot(points);
}

pub struct Config {
    pub filename: String,
    pub color_channel: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let color_channel = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a color channel"),
        };

        Ok(Config {
            filename,
            color_channel,
        })
    }
}

fn get_points(image: &str, color_channel: String) -> Vec<(f32, f32)> {
    let image = raster::open(image).unwrap();
    let mut points = vec![];
    for i in 0..440 {
        let pixel = image.get_pixel(7, i).unwrap();
        if color_channel.eq("r") {
            points.push((i as f32 * 0.265, pixel.r as f32));
        } else if color_channel.eq("g") {
            points.push((i as f32 * 0.265, pixel.g as f32));
        }
    }

    let mut min_y = 255.0;
    let mut max_y = 0.0;
    for point in points.iter() {
        if point.1 < min_y {
            min_y = point.1;
        }

        if point.1 > max_y {
            max_y = point.1;
        }
    }

    for i in 0..points.iter().len() {
        points[i].1 -= min_y;
        points[i].1 /= max_y - min_y;
    }

    points
}

fn plot(points: Vec<(f32, f32)>) {
    let root = BitMapBackend::new("images/plot.png", (1000, 300)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .margin_right(5)
        .margin_top(10)
        .build_cartesian_2d(0f32..120f32, 0f32..1.01f32)
        .unwrap();
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(15)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()
        .unwrap();
    chart
        .draw_series(PointSeries::of_element(points, 3, &RED, &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
        }))
        .unwrap();
}

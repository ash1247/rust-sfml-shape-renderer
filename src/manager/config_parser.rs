use std::fs;

use crate::settings;
use crate::entities::*;
use settings::config::Config;

pub struct ReturnValue(pub Config, pub Vec<circle::Circle>, pub Vec<rectangle::Rectangle>);

pub fn parse_config_string() -> ReturnValue {
    let file_path = "./config/config";
   
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut config = Config::new_empty();
    let mut circles: Vec<circle::Circle> = Vec::new();
    let mut rectangles: Vec<rectangle::Rectangle> = Vec::new();
    println!("{:#?}", lines);
    for line in lines {
        let settings: Vec<&str> = line.split("=").collect();
        
        match *settings.first().unwrap() {
            "window" => {
                let mut values: Vec<&str> = settings.last().unwrap().split("x").collect();
                if values.len() < 2 {
                    println!("Resolution parsing error. Default values loaded");
                    values.clear();
                    values.push("800");
                    values.push("600");
                }
                config.window_width = values[0].to_string().parse::<u32>().unwrap();
                config.window_height = values[1].to_string().parse::<u32>().unwrap();
            }
            "font" => {
                let value = *settings.last().unwrap();
                config.font_path = value.to_string();
            }
            "shape" => {
                let values: Vec<&str> = settings.last().unwrap().split(" ").collect();
                if values.len() >= 10 {
                    let name = values[1].to_string();

                    let x = values[2].to_string().parse::<f64>().unwrap();
                    let y = values[3].to_string().parse::<f64>().unwrap();
                    let position = position::Position::new(x, y);

                    let dx = values[4].to_string().parse::<f64>().unwrap();
                    let dy = values[5].to_string().parse::<f64>().unwrap();
                    let displacement = displacement::Displacement::new(dx, dy);

                    let r = values[6].to_string().parse::<u32>().unwrap();
                    let g = values[7].to_string().parse::<u32>().unwrap();
                    let b = values[8].to_string().parse::<u32>().unwrap();
                    let rgb_color = color::ColorRGB::new(r, g, b);

                    if values[0] == "circle" && values.len() == 10 {
                        
                        let radius = values[9].to_string().parse::<f64>().unwrap();

                        let circle = circle::Circle::new(
                                    name, 
                                    radius, 
                                    rgb_color, 
                                    position,
                                    displacement
                                );
                        circles.push(circle);

                    } else if values[0] == "rectangle" && values.len() == 11 {
                        let h =  values[9].to_string().parse::<f64>().unwrap();
                        let w =  values[10].to_string().parse::<f64>().unwrap();

                        let rectangle = rectangle::Rectangle::new(
                            name, 
                            h, w,
                            rgb_color, 
                            position,
                            displacement
                        );
                        rectangles.push(rectangle);
                    }
                }
            }
            _ => println!("This was not supposed to happen")
        }
    }
    return ReturnValue(config, circles, rectangles);
}
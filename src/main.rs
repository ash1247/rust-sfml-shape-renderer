mod settings;
mod entities;
mod manager;

use settings::config::Config;
use manager::config_parser::ReturnValue;
use entities::*;
use sfml::{
    graphics::{
        CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
        Transformable,
    },
    system::{Vector2f, Vector2},
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

static mut FONT: Option<sfml::SfBox<sfml::graphics::Font>> = None;

fn main() {   
    let values: ReturnValue = manager::config_parser::parse_config_string();

    let config: Config = values.0.clone(); 
    let mut circles = values.1.clone();
    let mut rectangles = values.2.clone();
    load_font_from_path(&config.font_path);
    let mut object_move_speed: f32 = 10.0;
    
    let mut context_settings = ContextSettings::default();
    context_settings.set_antialiasing_level(0);
    // Create a new window
    let mut window = RenderWindow::new(VideoMode::new(config.window_width, config.window_height, VideoMode::desktop_mode().bits_per_pixel), 
                    "SFML window",
                    Style::CLOSE,
                    &context_settings);
    // Limit the framerate to 60 frames per second (this step is optional)
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);
    println!("aa = {}", window.settings().antialiasing_level());

    let mut circle_views = get_circle_views(circles.clone());
    let mut circle_text_views: Vec<Text> = get_circle_text_views(circles.clone());

    let mut rectangle_views = get_rectangle_views(rectangles.clone());
    let mut rectangle_text_views: Vec<Text<'static>> = get_rectangle_text_views(rectangles.clone());
    
    let mut text = Text::new("SFML TESTING", get_font(), 24);
    text.set_position(Vector2f::new(0.0, (config.window_height - text.character_size()) as f32));
   
    // The main loop - ends as soon as the window is closed
    while window.is_open() {
        // Event processing
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed {code, ..} => {
                    println!("Key pressed");
                    if code == Key::X {
                        object_move_speed *= -1.0
                    }
                    if code == Key::F {
                        if object_move_speed <= 500.0 {
                            object_move_speed += 5.0;
                        }
                    }
                    if code == Key::D {
                        object_move_speed -= 5.0;
                    }
                }
                _ => ()
            }
        }

        // Activate the window for OpenGL rendering
        window.set_active(true);
        
        window.clear(Color::BLACK);
        // OpenGL drawing commands go here...
        for index in 0..circle_views.len() {
            let object = &mut circles[index];
            let view = &mut circle_views[index];
            let text = &mut circle_text_views[index];
            let collided = detected_collision(&window, view);
            get_new_position(object, view, text, object_move_speed, collided);
            window.draw(view);
            window.draw(text);
        }
        for index in 0..rectangle_views.len() {
            let object = &mut rectangles[index];
            let view = &mut rectangle_views[index];
            let text = &mut rectangle_text_views[index];
            let collided = detected_collision(&window, view);
            get_new_position(object, view, text, object_move_speed, collided);
            window.draw(view);
            window.draw(text);
        }
        window.draw(&text);
        if text.string().to_rust_string() == "" {

        }
        // End the current frame and display its contents on screen
        window.display();
    }
}

pub fn get_circle_views(circles: Vec<circle::Circle>) -> Vec<CircleShape<'static>> {
    let mut views: Vec<CircleShape> = Vec::new();
    for circle in circles {
        let mut circle_view = CircleShape::new(circle.radius as f32, 30);
        circle_view.set_fill_color(Color::rgb(circle.color.red as u8 , circle.color.green as u8, circle.color.blue as u8));
        circle_view.set_position(Vector2::new(circle.position.x as f32, circle.position.y as f32));
        views.push(circle_view);
    }
    return views;
}

pub fn get_circle_text_views(circles: Vec<circle::Circle>) -> Vec<Text<'static>> {
    let mut circle_text_views: Vec<Text<'static>> = Vec::new();
    for circle in circles {
        let name = circle.name;
        let mut text = Text::new(&name, get_font(), 18);
        text.set_position(Vector2::new(circle.position.x as f32 / 2.0, circle.position.y as f32/ 2.0));
        circle_text_views.push(text)
    }
    return circle_text_views
}

pub fn get_rectangle_views(rectangles: Vec<rectangle::Rectangle>) -> Vec<RectangleShape<'static>> {
    let mut views: Vec<RectangleShape> = Vec::new();
    for rectangle in rectangles {
        let mut rectangle_view = RectangleShape::with_size(Vector2::new(rectangle.height as f32, rectangle.width as f32));
        rectangle_view.set_fill_color(Color::rgb(rectangle.color.red as u8 , rectangle.color.green as u8, rectangle.color.blue as u8));
        rectangle_view.set_position(Vector2::new(rectangle.position.x as f32, rectangle.position.y as f32));
        views.push(rectangle_view);
    }
    return views;
}

pub fn get_rectangle_text_views(rectangles: Vec<rectangle::Rectangle>) -> Vec<Text<'static>> {
    let mut rectangle_text_views: Vec<Text<'static>> = Vec::new();
    for rectangle in rectangles {
        let name = rectangle.name;
        let mut text = Text::new(&name, get_font(), 18);
        text.set_position(Vector2::new(rectangle.position.x as f32 / 2.0, rectangle.position.y as f32/ 2.0));
        rectangle_text_views.push(text)
    }
    return rectangle_text_views
}

pub fn load_font_from_path(path: &String) {
    let font = Font::from_file(path).unwrap();
    unsafe {
        FONT = Option::from(font.clone());
        FONT.as_deref().unwrap_or(&font);
    }
}

pub fn get_font() -> &'static sfml::graphics::Font {
    unsafe {
        return FONT.as_deref().unwrap();
    }
        
}

pub fn get_new_position<S, T>(object: &mut S, view: &mut T, text: &mut Text<'static>, speed: f32, collided: (bool, bool)) where S: shapeobject::ShapeObject, T: Shape<'static> {
    let previous_position = view.position();
    let mut displacement_vector = object.get_displacement();
    let (collided_vertical, collided_horizontal) = collided;
    if collided_vertical {
        displacement_vector.dy *= -1.0;
    }
    if collided_horizontal {
        displacement_vector.dx *= -1.0;
    }
    object.set_displacement(displacement_vector); 
    let move_vector = Vector2f::new(displacement_vector.dx as f32 * speed, displacement_vector.dy as f32 * speed);
    let new_position = previous_position + move_vector;
    view.set_position(new_position);
    let view_local_bounds = view.local_bounds();
    let text_local_bounds = text.local_bounds();
    // println!("view_local_bounds(left:{}, top:{} width:{} height:{})", 
    //                             view_local_bounds.left, view_local_bounds.top, 
    //                             view_local_bounds.width, view_local_bounds.height);
    // println!("text_local_bounds(left:{}, top:{} width:{} height:{})", text_local_bounds.left, text_local_bounds.top, 
    //         text_local_bounds.width, text_local_bounds.height);
    text.set_position(Vector2::new(
        new_position.x - text_local_bounds.left + ((view_local_bounds.width - text_local_bounds.width) / 2.0), 
        new_position.y - text_local_bounds.top + ((view_local_bounds.height - text_local_bounds.height) / 2.0)));
}

pub fn detected_collision<T>(window: &RenderWindow, view: &T) -> (bool, bool) where T: Shape<'static> {
    let window_bounds = window.size();
    let view_local_bounds = view.local_bounds();
    let view_global_bounds = view.global_bounds();
    // println!("window_bounds(x:{}, y:{})\nlocal_bounds(left:{}, top:{} width:{} height:{})", window_bounds.x, window_bounds.y, 
                                // view_local_bounds.left, view_local_bounds.top, 
                                // view_local_bounds.width, view_local_bounds.height);
    // println!("global_bounds(left:{}, top:{} width:{} height:{})", view_global_bounds.left, view_global_bounds.top, 
                                            // view_global_bounds.width, view_global_bounds.height);
    
    let mut collided_horizontal = false;
    let mut collided_vertical = false;
    // top and bottom                                       
    if view_global_bounds.top <= 0.0 
            || view_global_bounds.top + view_global_bounds.height >= window_bounds.y as f32 {
        collided_vertical = true;
    }
    // left and right
    if view_global_bounds.left <= 0.0 || view_global_bounds.left + view_global_bounds.width >= window_bounds.x as f32 {
        collided_horizontal = true;
    }
    return (collided_vertical, collided_horizontal);
}

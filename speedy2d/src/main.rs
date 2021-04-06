  
use fltk::{
    app,
    enums::Event,
    prelude::{WidgetBase, WidgetExt, GroupExt, WindowExt},
    window::{GlutWindow, Window},
};
use image::*;
use image::io::Reader as ImageReader;
use speedy2d::GLRenderer;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageSmoothingMode};
use speedy2d::font::{TextLayout, TextOptions};
use dyn_clone::DynClone;
use std::cell::RefCell;
use std::rc::Rc;

type SpeedyColor = speedy2d::color::Color; 

//C++ tutorial 
// https://www.youtube.com/watch?v=ZQ8qtAizis4&t=574s
#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

pub trait CanvasItemExt:DynClone + std::fmt::Debug{
    fn coords(&self)->Vector2<f32>;
    fn set_coords(&mut self, coords: Vector2<f32>);
    fn image_data(&self)->Option<CanvasImage>;
    fn draw(&self,  graphics: &mut speedy2d::Graphics2D, c_offset: Vector2<f32>, c_scale: Vector2<f32>);
}
dyn_clone::clone_trait_object!(CanvasItemExt);

pub type CanvasItem = Box<dyn CanvasItemExt>;
#[derive(Clone, Debug)]
pub struct CanvasSettings{
    pub offset: Vector2<f32>,
    pub items: Vec<CanvasItem>,
    pub screen_pos: Vector2<i32>,
    pub world_pos: Vector2<f32>,
    pub drag_coord: Vector2<i32>,
    pub drag_start: Vector2<i32>,
    pub zoom_scale: Vector2<f32>,
}

type CanvasRef = Rc<RefCell<CanvasSettings>>;
#[derive(Clone)]
struct CanvasContainer(CanvasRef);

impl CanvasContainer{
    fn new(inner: CanvasSettings)->CanvasContainer{
        let canvas = CanvasSettings{ 
            offset: inner.offset.clone(),
            items: inner.items.clone(),
            screen_pos: inner.screen_pos.clone(),
            world_pos: inner.world_pos.clone(),
            drag_coord: inner.drag_coord.clone(),
            drag_start: inner.drag_start.clone(),
            zoom_scale: inner.zoom_scale.clone(),
        };
        CanvasContainer(Rc::new(RefCell::new(canvas)))
    }
    fn zoom(&self, ev_pos_x: i32, ev_pos_y: i32, zoom_scale: f32){
        let mut scale = (self.0.borrow_mut()).zoom_scale;
        let off = (self.0.borrow_mut()).offset;
        let before_zoom_coord = screen_to_world_2(Vector2::new(ev_pos_x, ev_pos_y), off, scale);
        (self.0.borrow_mut()).zoom_scale.x *= zoom_scale;
        (self.0.borrow_mut()).zoom_scale.y *= zoom_scale;
        scale.x *= zoom_scale;
        scale.y *= zoom_scale;
        let after_zoom = screen_to_world_2(Vector2::new(ev_pos_x, ev_pos_y), off, scale);
        (self.0.borrow_mut()).offset.x += before_zoom_coord.x - after_zoom.x;
        (self.0.borrow_mut()).offset.y += before_zoom_coord.y - after_zoom.y;
    }
    
    fn nudge_left(&self){
        (self.0.borrow_mut()).offset.x -= 1.0
    }
    fn nudge_right(&self){
        (self.0.borrow_mut()).offset.x += 1.0
    }
    fn nudge_up(&self){
        (self.0.borrow_mut()).offset.y -= 1.0
    }
    fn nudge_down(&self){
        (self.0.borrow_mut()).offset.y += 1.0
    }
    fn pan_screen(&self, drag_x: i32, drag_y: i32){
        (self.0.borrow_mut()).drag_coord = Vector2::new(drag_x, drag_y);
        let scale = (self.0.borrow_mut()).zoom_scale;
        let delta_x =  ((self.0.borrow_mut()).drag_start.x as f32 - drag_x as f32)/ scale.x;
        let delta_y =  ((self.0.borrow_mut()).drag_start.y as f32 - drag_y as f32)/scale.y;

        (self.0.borrow_mut()).offset.x += delta_x; 
        (self.0.borrow_mut()).offset.y += delta_y; 

        self.0.borrow_mut().drag_start.x = drag_x;
        self.0.borrow_mut().drag_start.y = drag_y;
    }
    fn set_drag_start(&self, start_x: i32, start_y: i32){
        (self.0.borrow_mut()).drag_start = Vector2::new(start_x, start_y);
    }
}

#[derive(Clone)]
pub struct CanvasImage{
    data: Vec<u8>,
    width: u32,
    height: u32,
    pos: Vector2<f32>,
    scale: f32, 
    img_data_type: speedy2d::image::ImageDataType,
}
impl CanvasImage{
    pub fn new(path: &str, pos: Vector2<f32>, scale: f32) -> Self{
        let loaded_img: DynamicImage = ImageReader::open(path).unwrap().decode().unwrap();
        let has_alpha = has_alpha(&loaded_img);
        let ci_bytes = match has_alpha {
            true=>loaded_img.into_rgba8().into_raw(),
            false=>loaded_img.into_rgb8().into_raw(),
        };
        
        // let ci_bytes: Vec<u8> = ImageReader::open(path).unwrap().decode().unwrap().to_bytes();
        let img_type = match has_alpha{
            true=>speedy2d::image::ImageDataType::RGBA,
            false=>speedy2d::image::ImageDataType::RGB,
        };
        
       
        let (ci_width, ci_height) =  ImageReader::open(path).unwrap().decode().unwrap().dimensions(); 
        let ci = CanvasImage{
            data: ci_bytes,
            width: ci_width,
            height: ci_height,
            pos: pos,
            scale: scale,
            img_data_type: img_type, 
        };
        ci
    }
}
impl std::fmt::Debug for CanvasImage{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Canvas Image: data len {}, width: {}, height: {}, pos: {}, {}", self.data.len(), self.width, self.height, self.pos.x, self.pos.y)
    }   
}

impl CanvasItemExt for CanvasImage{
    fn coords(&self)->Vector2<f32>{
        self.pos
    }
    fn set_coords(&mut self, coords: Vector2<f32>){
        self.pos = coords
    }
    fn image_data(&self)->Option<CanvasImage>{
        Some(self.clone())
    }
    fn draw(&self,  graphics: &mut speedy2d::Graphics2D, c_offset: Vector2<f32>, c_scale: Vector2<f32>){
        let coords = self.pos;
        let bytes = &self.data;
        let img_width = self.width;
        let img_height = self.height;
        let img_type = self.img_data_type.clone();
        let rect_pos = world_to_screen_2(coords, c_offset, c_scale);
        let rect_pos_2 = world_to_screen_2(Vector2::new(coords.x+img_width as f32, coords.y+img_height as f32), c_offset, c_scale);

        let to_draw_image = match graphics.create_image_from_raw_pixels(
            img_type,
            ImageSmoothingMode::NearestNeighbor,
            Vector2::new(img_width, img_height),
            &bytes,
        ){
            Ok(img) => img,
            Err(e) => panic!("Error creating image {}", e)
        };
        let rect = speedy2d::shape::Rectangle::new(
            Vector2::new(rect_pos.x as f32, rect_pos.y as f32), 
            Vector2::new(rect_pos_2.x as f32, rect_pos_2.y as f32)
        );

        graphics.draw_rectangle_image(rect, &to_draw_image);
    }
}
#[derive(Clone)]
pub struct CanvasCircle{
    pos: Vector2<f32>,
    radius: f32,
    fill: SpeedyColor,
}
impl CanvasCircle{
    pub fn new(pos: Vector2<f32>, radius: f32, fill: SpeedyColor)->Self{
        let cc = CanvasCircle{
            pos: pos.clone(),
            radius: radius,
            fill: fill,
        };
        cc
    }
}
impl std::fmt::Debug for CanvasCircle{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Canvas Cirlce: radius: {}, pos: {}, {}",self.radius, self.pos.x, self.pos.y)
    }   
}
impl CanvasItemExt for CanvasCircle{
    fn coords(&self)->Vector2<f32>{
        self.pos
    }
    fn set_coords(&mut self, coords: Vector2<f32>){
        self.pos = coords
    }
    fn image_data(&self)->Option<CanvasImage>{
        None
    }
    fn draw(&self,  graphics: &mut speedy2d::Graphics2D, c_offset: Vector2<f32>, c_scale: Vector2<f32>){
        let coords = self.pos;
        let radius = self.radius; 
        
        let circle_pos = world_to_screen_2(coords, c_offset, c_scale);
        let final_pos = Vector2::new(circle_pos.x as f32, circle_pos.y as f32);
        graphics.draw_circle(final_pos, radius*c_scale.x, self.fill);
    }
}

fn world_to_screen_2(world: Vector2<f32>, offset: Vector2<f32>, scale: Vector2<f32>)->Vector2<i32>{
    let screen_x = ((world.x - offset.x) * scale.x) as i32;
    let screen_y = ((world.y - offset.y) * scale.y) as i32;
    Vector2::new(screen_x, screen_y)
}
fn screen_to_world_2(screen: Vector2<i32>, offset: Vector2<f32>, scale: Vector2<f32>)->Vector2<f32>{
    let world_x = (screen.x as f32 / scale.x) - offset.x;
    let world_y = (screen.y as f32 / scale.y) - offset.y;
    Vector2::new(world_x, world_y)
}

fn main() {
    let app = app::App::default();
    app.set_visual(fltk::enums::Mode::Alpha).unwrap();
    let wind_size =  500;
    let mut main_win = Window::new(450,450,wind_size, wind_size, "Speedy 2d Scroll Zoom Canvas");
    let mut win = GlutWindow::default().with_size(wind_size, wind_size).with_pos(0,0);
    win.end();
    main_win.end();
    main_win.show();
    win.make_current();

    let helvetica_bytes = include_bytes!("../HelveticaNeue/HelveticaNeue-Medium.otf");
    let helvetica_font = speedy2d::font::Font::new(helvetica_bytes).unwrap();
    
    gl::load_with(|s| win.get_proc_address(s));
    let mut renderer = unsafe { GLRenderer::new_for_current_context((wind_size as u32, wind_size as u32)) }.unwrap();

    let container = CanvasContainer::new(CanvasSettings{
        offset: Vector2::new(0.0, 0.0),
        items: vec![
            Box::new(CanvasCircle::new(Vector2::new(-100.0, 300.0), 300.0, Color::BLUE)),   
            Box::new(CanvasImage::new("imgs/forest.jpg", Vector2::new(000.0, 100.0), 0.5)),
            Box::new(CanvasImage::new("imgs/smiley.png", Vector2::new(-200.0, 100.0), 0.5)),
            Box::new(CanvasImage::new("imgs/rust-logo.png", Vector2::new(300.0, 500.0), 0.5)),
            Box::new(CanvasCircle::new(Vector2::new(200.0, 200.0), 100.0, Color::RED)),   
        ],
        screen_pos: Vector2::new(0, 0),
        world_pos: Vector2::new(0.0, 0.0),
        drag_coord: Vector2::new(0, 0),
        drag_start: Vector2::new(0,0),
        zoom_scale: Vector2::new(0.5, 0.5),
    });

    let container_rc_handle = container.clone();
    let mut win_cl = win.clone();
    win.handle(Box::new(move |ev| match ev {
        Event::Push => {
            println!("Pushed");
            container_rc_handle.set_drag_start(app::event_x(), app::event_y());
            true
        },
        Event::MouseWheel=>{
            if app::event_dy() < 0 {
                container_rc_handle.zoom(app::event_x_root() - main_win.x(), app::event_y_root() - main_win.y(), 1.05);
            } else {
                container_rc_handle.zoom(app::event_x_root() - main_win.x(), app::event_y_root() - main_win.y(), 0.95);
            }
            win_cl.redraw();
            false
        }
        Event::KeyUp=>{
            match app::event_key(){
                fltk::enums::Key::Right =>{
                    container_rc_handle.nudge_right();
                    win_cl.redraw();
                }
                fltk::enums::Key::Left =>{
                    container_rc_handle.nudge_left();
                    win_cl.redraw();
                }
                fltk::enums::Key::Up =>{
                    container_rc_handle.nudge_up();
                    win_cl.redraw();
                }
                fltk::enums::Key::Down =>{
                    container_rc_handle.nudge_down();
                    win_cl.redraw();
                }
                _=>()
            }
            true
        }
        Event::Drag=>{
            dbg!(app::event_x(), app::event_y());
            container_rc_handle.pan_screen(app::event_x(), app::event_y());
            win_cl.redraw();
            true
        }
        _ => false,
    }));

    let container_rc = container.clone();
    win.draw(Box::new(move||{
        renderer.draw_frame(|graphics| {
            let font_size = 15.0;
            let scene = container_rc.0.borrow();
            graphics.clear_screen(Color::WHITE);
            for x in &scene.items{
                x.draw(graphics, scene.offset, scene.zoom_scale);
            };

            let zoom_text = helvetica_font.layout_text(&format!("Zoom: x: {}, y: {}", format!("{:.1$}", scene.zoom_scale.x.to_string(), 4),  format!("{:.1$}", scene.zoom_scale.y.to_string(), 4)), font_size, TextOptions::new());
            let event_text = helvetica_font.layout_text(&format!("Event: x: {}, y: {}", app::event_x(), app::event_y()), font_size, TextOptions::new());
            let offset_text = helvetica_font.layout_text(&format!("Offset: x: {}, y: {}", format!("{:.1$}", scene.offset.x.to_string(), 5),  format!("{:.1$}", scene.offset.y.to_string(), 5)), font_size, TextOptions::new());
            
            graphics.draw_text((0.0, wind_size as f32 - font_size), Color::BLACK, &zoom_text);
            graphics.draw_text((180.0, wind_size as f32 - font_size), Color::BLACK, &event_text);
            graphics.draw_text((360.0, wind_size as f32 - font_size), Color::BLACK, &offset_text);
        });

    }));

    app.run().unwrap();
}

fn has_alpha(img: &DynamicImage) -> bool {
    img.color().has_alpha()
}
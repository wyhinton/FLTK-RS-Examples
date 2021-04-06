  
use fltk::{
    app,
    enums::Event,
    prelude::{WidgetBase, WidgetExt, GroupExt, WindowExt},
    window::{GlutWindow, Window},
    utils
};
use image::*;
use image::io::Reader as ImageReader;
use speedy2d::GLRenderer;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageDataType, ImageSmoothingMode};
use dyn_clone::DynClone;
use std::cell::RefCell;
use std::cell::Ref;
use std::rc::Rc;
use std::ops::Deref;


#[derive(Debug, Clone)]
pub enum Message {
    Test,   
    ZoomIn,
    ZoomOut,
}

// pub trait CanvasItemExt:DynClone + Debug{
pub trait CanvasItemExt:DynClone{
    fn coords(&self)->Vector2<f32>;
    fn set_coords(&mut self, coords: Vector2<f32>);
    fn image_data(&self)->CanvasImage;
}
dyn_clone::clone_trait_object!(CanvasItemExt);

pub type CanvasItem = Box<dyn CanvasItemExt>;
#[derive(Clone)]
pub struct CanvasSettings{
    pub zoom_level: f32, 
    pub offset: (f32, f32),
    pub items: Vec<CanvasItem>,
}

// impl CanvasSettings {
//     pub fn set_zoom(&mut self, zoom: f32){
//         self.zoom_level = zoom
//     }
//     pub fn get_settings(&self)->AppData{
//         AppData{
//             data: CanvasSettings{
//                 zoom_level: self.zoom_level.borrow(),
//                 offset: self.offset.borrow(),
//                 items: self.items.borrow(),
//         }
//         }
//     }
// }
#[derive(Clone)]
pub struct CanvasImage{
    data: Vec<u8>,
    width: u32,
    height: u32,
    pos: Vector2<f32>,
}

pub struct AppData<'a>{
// pub struct AppData<'a>{
    data: Ref<'a, CanvasSettings>
}
// pub struct AppData<'a>{
//     data: Rc<RefCell<'a>>
// }
impl AppData {
    // fn new(t: Ref<T>) -> AppData {
    // // fn new(t: &Rc<RefCell<T>>) -> AppData<T> {
    //     AppData { data: t.clone() }
    // }

    fn dup(&self) -> AppData {
        AppData {
            data: self.data.clone(),
        }
    }
    fn data(&self)->AppData{
        AppData{
            data: self.data.borrow()
        }
        // Ref<self.data>
    }
    // fn zoom_level(&self) -> Vector2<f32> {
    //     let test: CanvasSettings = *self.data.borrow_mut();
    //     test.zoom_level;
    //     // AppData {
    //     //     data: self.data.clone(),
    //     // }
    // }
}
impl <'b> Deref for AppData<'b> {
    type Target = CanvasSettings;
    
    fn deref(&self) -> &CanvasSettings {
        &self.data
    }
}

impl CanvasImage{
    pub fn new(path: &str, pos: Vector2<f32>) -> Self{
        let ci_bytes: Vec<u8> = ImageReader::open(path).unwrap().decode().unwrap().to_bytes();
        let (ci_width, ci_height) =  ImageReader::open(path).unwrap().decode().unwrap().dimensions(); 
        let ci = CanvasImage{
            data: ci_bytes,
            width: ci_width,
            height: ci_height,
            pos: pos,
        };
        ci
    }
    // pub fn new(path: &str, pos: Vector2<f32>) -> Self{
    //     let ci_bytes: Vec<u8> = ImageReader::open(path).unwrap().decode().unwrap().to_bytes();
    //     let (ci_width, ci_height) =  ImageReader::open(path).unwrap().decode().unwrap().dimensions(); 
    //     let ci = CanvasImage{
    //         data: ci_bytes,
    //         width: ci_width,
    //         height: ci_height,
    //         pos: pos,
    //     };
    //     ci
    // }
}

impl CanvasItemExt for CanvasImage{
    fn coords(&self)->Vector2<f32>{
        self.pos
    }
    fn set_coords(&mut self, coords: Vector2<f32>){
        self.pos = coords
    }
    fn image_data(&self)->CanvasImage{
        self.clone()
    }
}


fn main() {

    let app = app::App::default();
    dbg!(app::screen_scale(0));
    dbg!(app::screen_scale(1));
    let wind_size = (app::screen_scale(1) * 200.0) as i32; 
    let (s, r) = app::channel::<Message>();
    let mut main_win = Window::new(200,200,wind_size, wind_size, "");
    let mut win = GlutWindow::default().with_size(450, 450).center_of(&main_win);
    win.end();
    main_win.end();
    main_win.show();
    win.make_current();
    let s_win = s.clone();

    win.handle(Box::new(move |ev| match ev {
        Event::Push => {
            println!("Pushed");
            true
        },
        Event::MouseWheel=>{
            if app::event_dy() < 0 {
                dbg!("scroll up");
                s_win.send(Message::ZoomIn);
            } else {
                dbg!("scroll down");
                s_win.send(Message::ZoomOut);
            }
            // s_win.send(Message::Test);
            true
        }
        Event::KeyUp=>{
            match app::event_key(){
                fltk::enums::Key::Right =>{
                    dbg!("got right arrow");
                }
                _=>()
            }
            true
        }
        _ => false,
    }));
    gl::load_with(|s| win.get_proc_address(s));

    let mut renderer = unsafe { GLRenderer::new_for_current_context((300, 300)) }.unwrap();

    let img: Vec<u8> = ImageReader::open("imgs/2.jpg").unwrap().decode().unwrap().to_bytes();
    let (img_width, img_height) =  ImageReader::open("imgs/2.jpg").unwrap().decode().unwrap().dimensions(); 

    
    let mut settings = Rc::from(RefCell::from(CanvasSettings{
        zoom_level: 1.0,
        offset: (0.0, 0.0),
        items: vec![Box::new(CanvasImage::new("imgs/2.jpg", Vector2::new(0.0, 0.0)))],
    }));
    let app_data = AppData::new(&settings).dup();


    // let settings_rc = app_data.data.clone();
    // let settings_rc = settings.clone();
    win.draw(Box::new(move||{
        renderer.draw_frame(|graphics| {
            let stest = &settings_rc.borrow();
            let first_image = &stest.items[0].image_data();
            let to_draw_image = match graphics.create_image_from_raw_pixels(
                ImageDataType::RGB,
                ImageSmoothingMode::NearestNeighbor,
                Vector2::new(first_image.width,first_image.height),
                &first_image.data,
            ){
                Ok(img) => img,
                Err(e) => panic!("Error creating image {}", e)
            };
            let rect = speedy2d::shape::Rectangle::new(stest.items[0].coords(), Vector2::new(200. * stest.zoom_level, 200. * stest.zoom_level));
            graphics.draw_rectangle_image(rect, &to_draw_image);
        });
    }));



    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    println!("{}", "got test message");
                    app::redraw();
                }
                ZoomIn=>{
                    // settings.zoom_level += 0.1;
                    // *Rc::get_mut(&mut settings).unwrap() = CanvasSettings{
                    //     zoom_level: settings.zoom_level += 0.1,
                    //     offset:  settings.offset.clone(),
                    //     items:  settings.items.clone(),
                    // };
                    *settings.borrow_mut() = CanvasSettings{
                        zoom_level: settings.zoom_level += 0.1,
                        offset:  settings.offset.clone(),
                        items:  settings.items.clone(),
                    };
                    let mut test = *settings.borrow_mut();
                    test.set_zoom(2.0);

                    // test.zoom_level += 0.1;
                    win.redraw();
                    dbg!("going to zoom in");
                }
                ZoomOut=>{
                    // let mut test = *settings.borrow_mut();
                    // test.zoom_level -= 0.1;
                    win.redraw();
                }
                
            }
        }
    }
}

    // win.draw(move||{
    //     renderer.draw_frame(|graphics| {
    //         // let first_image = &settings.items[0].image_data();
    //         let first_image = &tt.items[0].image_data();
    //         let to_draw_image = match graphics.create_image_from_raw_pixels(
    //             ImageDataType::RGB,
    //             ImageSmoothingMode::NearestNeighbor,
    //             Vector2::new(first_image.width,first_image.height),
    //             &first_image.data,
    //         ){
    //             Ok(img) => img,
    //             Err(e) => panic!("Error creating image {}", e)
    //         };
    //         // let rect = speedy2d::shape::Rectangle::new(settings.items[0].coords(), Vector2::new(200. * settings.zoom_level, 200. * settings.zoom_level));
    //         let rect = speedy2d::shape::Rectangle::new(tt.items[0].coords(), Vector2::new(200. * tt.zoom_level, 200. * tt.zoom_level));
    //         graphics.draw_rectangle_image(rect, &to_draw_image);
    //     });
    // });

    // renderer.draw_frame(|graphics| {
    //     let first_image = &settings.items[0].image_data();
    //     // let first_image = &tt.items[0].image_data();
    //     let to_draw_image = match graphics.create_image_from_raw_pixels(
    //         ImageDataType::RGB,
    //         ImageSmoothingMode::NearestNeighbor,
    //         Vector2::new(first_image.width,first_image.height),
    //         &first_image.data,
    //     ){
    //         Ok(img) => img,
    //         Err(e) => panic!("Error creating image {}", e)
    //     };
    //     let rect = speedy2d::shape::Rectangle::new(settings.items[0].coords(), Vector2::new(200. * settings.zoom_level, 200. * settings.zoom_level));
    //     // let rect = speedy2d::shape::Rectangle::new(tt.items[0].coords(), Vector2::new(200. * settings.zoom_level, 200. * settings.zoom_level));
    //     graphics.draw_rectangle_image(rect, &to_draw_image);
    // });
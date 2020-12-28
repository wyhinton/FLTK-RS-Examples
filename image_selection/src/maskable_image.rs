// use ftlk::*, 
use fltk::{app, image::*, frame::*, group::*, input::*, draw::*};
use std::ops::{Deref, DerefMut};
use std::path::Path;
// use syn::*;
// use 
use super::Message;
use std::cell::RefCell;
use std::rc::Rc;
use image::imageops::*;
use image::GenericImageView;



use crate::utils;
#[derive(Clone)]
pub struct CropSegment{
    pub x: i32, 
    pub y: i32, 
    pub width: i32, 
    pub height: i32,
    pub img: Vec<u8>,
    // img: fltk::image::RgbImage,
}

impl CropSegment {
    // pub fn new(x: i32, y: i32, width: i32, height: i32, cs_img: fltk::image::RgbImage) -> Self {
    pub fn new(x: i32, y: i32, width: i32, height: i32, cs_img: Vec<u8>) -> Self {
        let mut cs = CropSegment{
            x: x,
            y: y, 
            width: width,
            height: height,
            img: cs_img, 
        };
        cs
    }
}
pub struct MaskableImage {
    // pub pack: Pack, 
    pub frame: Frame,
}


impl MaskableImage {
    pub fn new(x: i32, y: i32, w: i32, h: i32, img_path: &str, s: fltk::app::Sender<Message>) -> Self {
        let icon_w = 24; 
        let input_w = w-icon_w;
        let mut sb = MaskableImage {
            // pack: Pack::new(x,y,w,h,""),
            frame: Frame::new(x,y,w,h,"")
        };
        sb.frame.set_frame(FrameType::BorderBox);
        // sb.frame.set_frame(FrameType::FlatBox);
        sb.frame.set_color(Color::Red);
        // utils::debug_widg::<fltk::widget::Widget>(&sb.frame.into_widget();


        let mut img = image::open(img_path).unwrap();
        // println!("image depth is {}", img.depth() );
        println!("w is: {}, h is: {}", img.width(), img.height());
        let (x, y) = img.dimensions();

        let mut grey_img = grayscale(&img); 
        let rgb_image = RgbImage::new(&img.to_bytes(), x, y, 4).unwrap();

        let mut s_x = 0;
        let mut s_y = 0;
        let mut f_x = 0;
        let mut f_y = 0; 
        let mut d_x = 0;
        let mut d_y = 0; 
        let mut released = false; 

        sb.frame.handle2(move |t, ev| {
            let mut grey_img_c = grey_img.clone();
            let s_clone = s.clone(); 
            let mut rgc = rgb_image.clone();
            let (ex,ey) = app::event_coords(); 
            let is_inside = utils::inside_widget::<fltk::widget::Widget>(&t.into_widget(), ex, ey);

            // if is_inside {
                match ev {
                    Event::Push =>{
                        let (tx, ty) = app::event_coords();
                        s_x = tx;
                        s_y = ty;
                        println!("sx {}, sy {}", s_x, s_y);
                        println!("fx {}, fy {}", f_x, f_y);
                        released = false; 
                        true
                    }
                    _=>false 
                };
                match ev{
                    Event::Released =>{
                        let (tx, ty) = app::event_coords();
                        if tx < t.x()  {
                            // unimplemented!();
                            // t.x()
                            f_x = t.x();
                        }
                        else if (tx >= t.x() + t.width()){
                            f_x = t.x() + t.width();
                        }
                        else if (ty >= t.y() + t.height()){
                            f_y  = t.y() + t.height();
                        } else {
                            f_x = tx;
                            f_y = ty;
                        };
                        released = true; 
                        true
                    }
                    _=>false 
                };
                match ev{
                    Event::Drag =>{
                        
                        let (tx, ty) = app::event_coords();
                        if tx <= t.x()  {
                            f_x = t.x();
                        } else if ty <= t.y(){
                            f_y = t.y();
                        }
                        else if (tx >= t.x() + t.width()){
                            f_x = t.x() + t.width();
                        }
                        else if (ty >= t.y() + t.height()){
                            f_y  = t.y() + t.height();
                        } else {
                            f_x = tx;
                            f_y = ty;
                        };
                        released = false; 
                        true
                    }
                    _=>false 
                };
            // }

            t.draw2(move |b|{
                //draw the base image in the frame
                rgc.draw(b.x(), b.y(), x as i32,y as i32);
                set_draw_color(Color::Yellow);
                let coverupw = (f_x-s_x).abs() as u32;
                let coveruph = (f_y-s_y).abs() as u32;
                // println!("cw {}, ch {}", coverupw, coveruph);

                if coverupw > 0 && coveruph > 0 {
                    if (f_x - s_x) < 0 {
                        println!("x pos is {}, y pos is {}", b.x(), b.y());
                        let my_crop_data = crop(&mut grey_img_c,(f_x-b.x()) as u32,(f_y-b.y()) as u32,coverupw,coveruph).to_image().to_vec();
                        // let my_crop_data = crop(&mut grey_img_c,(f_x-350) as u32,(f_y-350) as u32,coverupw,coveruph).to_image().to_vec();
                        let mut my_crop = RgbImage::new(&my_crop_data, coverupw, coveruph, 1).unwrap();
                        my_crop.draw(f_x, f_y, coverupw as i32, coveruph as i32);
                        draw_rect(f_x, f_y, (f_x-s_x).abs(), (f_y-s_y).abs());
                        set_cursor(Cursor::Default);
                        let ttt = CropSegment::new(f_x, f_y, coverupw as i32, coveruph as i32, my_crop_data);
       
                        if released {
                            unimplemented!();
                            s_clone.send(Message::UpdatePreviewSegment(ttt));
                            released = false;
                            println!("got here 1 {}", "" );
                        }

                    } else {
                        let my_crop_data = crop(&mut grey_img_c,(s_x-b.x()) as u32,(s_y-b.y()) as u32,coverupw,coveruph).to_image().to_vec();
                        let tbb =  crop(&mut grey_img_c,(s_x-b.x()) as u32,(s_y-b.y()) as u32,coverupw,coveruph).to_image();
                        let mut my_crop = RgbImage::new(&my_crop_data, coverupw, coveruph, 1).unwrap();
                        my_crop.draw(s_x, s_y, coverupw as i32, coveruph as i32);
                        draw_rect(s_x, s_y, (f_x-s_x).abs(), (f_y-s_y).abs());
                        set_cursor(Cursor::Default);
                        let ttt = CropSegment::new(s_x, s_y, coverupw as i32, coveruph as i32, my_crop_data);
                        println!("released is {}", released);
                        if released {
                            println!("got here 2 {}", "" );
                            // unimplemented!();
                            s_clone.send(Message::UpdatePreviewSegment(ttt));
                            released = false;
                        }
                        // s_clone.send(Message::UpdatePreviewSegment(ttt));
                    }
                }
            });
            t.redraw();
            true
        });
            
        sb
    }
}

impl Deref for MaskableImage {
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        // &self.pack
        &self.frame
    }
}

impl DerefMut for MaskableImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // &mut self.pack
        &mut self.frame
    }
}


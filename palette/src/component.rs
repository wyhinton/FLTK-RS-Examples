use fltk::{app::*, image::*, frame::*, group::*, draw::*};
use std::ops::{Deref, DerefMut};
use palette::*;
// ues std::fmt

#[derive(Clone)]
pub struct Component {
    pub pack: Pack, 
    pub frame: Frame,
}


trait ColorExt{
    fn to_rgb(&self)->(u8, u8, u8)
}

impl ColorExt for fltk::enums::Color{
    fn to_rgb(&self)->(u8, u8, u8){
        match *self {
            Color::ForeGround => (0,0,0),
            Color::BackGround => (0,0,0),
            Color::Inactive => (0,0,0),
            Color::Selection => (0,0,0),
            Color::Gray0 => (0,0,0),
            Color::Dark3 => (0,0,0),
            Color::Dark2 => (0,0,0),
            Color::Dark1 =>(0,0,0),
            Color::FrameDefault => (0,0,0),
            Color::Light1 => (0,0,0),
            Color::Light2 =>(0,0,0),
            Color::Light3 =>(0,0,0),
            Color::Black => (0,0,0),
            Color::Red =>(0,0,0),
            Color::Green => (0,0,0),
            Color::Yellow => (0,0,0),
            Color::Blue => (0,0,0),
            Color::Magenta => (0,0,0),
            Color::Cyan => (0,0,0),
            Color::DarkRed => (0,0,0),
            Color::DarkGreen => (0,0,0),
            Color::DarkYellow =>(0,0,0),
            Color::DarkBlue =>(0,0,0),
            Color::DarkMagenta =>(0,0,0),
            Color::DarkCyan =>(0,0,0)
            Color::White => (0,0,0),
        }
    }
}

impl Component {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut pack = Pack::new(x,y,w,h,"");
        pack.end();
        
        
   
        let mut frame1 = Frame::new(0,0,100,100,"");
        frame1.set_frame(FrameType::FlatBox);
        pack.add(&frame1);
        
        set_custom_color(&mut frame1, 200,0,0);
        // set_custom_color(&mut frame1, palette::rgb::Rgb::new(200,0,0));
        // set_custom_color(frame1, palette::rgb::)
        let mut sb = Component {
            pack: pack,
            frame: frame1,
        };
        // let frame1

            
        sb
    }
}

fn set_custom_color<W>(widg: &mut W, r: u8, g: u8, b: u8)
// fn set_custom_color<W>(widg: &mut W, col: palette::rgb::Rgb)
where
    W: WidgetExt,
{
    // let (r,g,b) = col.into_components();
    dbg!(r);
    dbg!(g);
    dbg!(b);
    widg.set_color(Color::from_rgb(r,g,b));
    widg.redraw();
}

impl Deref for Component {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for Component {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}
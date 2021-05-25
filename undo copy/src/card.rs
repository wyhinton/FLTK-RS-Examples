use fltk::{app::*, image::*, frame::*, group::*, draw, button::*};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::error::Error;
use crate::{GALLERY_HEIGHT, IMAGE_HEIGHT, Message, CHANNEL};
use uuid::Uuid;

#[derive(Clone)]
pub struct Card {
    pub pack: Pack,
    pub frame: Frame,
    pub id: Uuid
}

impl std::fmt::Debug for Card{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result{
         fmt.debug_struct("Card")
         .field("id", &self.id)
         .finish()
    }
}

static BUTTON_HEIGHT: i32 = 20; 
impl Card {
    pub fn new(img_path: &str) -> Result<Self, Box<dyn Error>>  {
        let s = CHANNEL.0.clone();
        let mut card_container = Pack::new(0,0,100,300,"");
        let mut img_frame = Frame::new(0,0,100,100,"");
        let mut delete_button = Button::new(0,0,100,BUTTON_HEIGHT,"X");
        delete_button.set_color(Color::Red);
        card_container.end();
        let id = Uuid::new_v4();
        delete_button.emit(s, Message::DeleteCard(id));
        let img = ImageReader::open(img_path)?
            .with_guessed_format()?
            .decode()?;
        let resized = img.resize(400,IMAGE_HEIGHT, image::imageops::FilterType::Triangle);
        let (w,h) = resized.dimensions();
        card_container.resize(img_frame.x(), img_frame.y(), w as i32, IMAGE_HEIGHT as i32 + BUTTON_HEIGHT);
        img_frame.resize(img_frame.x(), img_frame.y(), w as i32, IMAGE_HEIGHT as i32);
        img_frame.redraw();
        img_frame.draw2(move |b|{
            draw::draw_image(&resized.to_rgb8(), b.x(), b.y(), w as i32, h as i32, ColorDepth::Rgb8).unwrap();
        });

        let mut sb = Card {
            pack: card_container,
            frame: img_frame,
            id: id,
        };
        sb.frame.redraw();
            
        Ok(sb)
    }
    pub fn delete(&mut self){
        // self.pack.delete();
        fltk::WidgetBase::delete(self.pack.clone());
    }
}


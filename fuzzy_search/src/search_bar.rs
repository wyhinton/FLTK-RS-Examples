use fltk::{ app, frame::*, prelude::*, image::*, group::*, input::*, draw::*, enums::*};
use crate::CustomEvent;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, Clone, Ord, PartialEq, PartialOrd)]
pub struct Result {
    pub score: isize,
    pub val: String,
}

pub struct SearchBar {
    pub pack: Pack, 
    pub input: Input,
    pub value: Rc<RefCell<String>>,
}

impl SearchBar {
    pub fn new() -> Self {
        //initialize widgets
        
        let search_val = Rc::from(RefCell::from("".to_string()));
        let mut container = Pack::new(0,0,0,25, None);
        let icon = Icon::new();
        
        //make our text input fill up the right amount of space
        let input_w = icon.parent().unwrap().parent().unwrap().width() - icon.width();
        let mut text_input = Input::new(0,0,input_w,25, None);

        //if our text input is not empty, draw some place-holder text
        text_input.draw(move |b|{
            set_draw_color(Color::Black);
            if b.value().is_empty() {
                draw_text2("Search...", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
            }

        });
        container.end(); 
        container.set_type(PackType::Horizontal);
        container.set_frame(FrameType::BorderFrame);
        container.set_color(Color::Black);

        let val_cl = Rc::clone(&search_val);
        text_input.handle(move |t, ev| {
            match ev {
                Event::KeyDown =>{
                    println!("{}", "got a key down" );
                    *val_cl.borrow_mut() = t.value();
                    let _ = app::handle_main(CustomEvent::SEARCH_INPUT).unwrap();
                    true
                }
                _=>false 
            }
        });

        let sb = SearchBar {
            pack: container,
            input: text_input,
            value:  search_val,
        };
        sb
    }
    pub fn value(&self)->String{
        (*self.value.borrow().to_owned()).to_string()
    }
    
}

struct Icon{
    frame: Frame
}

impl Icon{
    pub fn new()->Self{
        let icon_w = 24; 
        let h = 24; 
        let mut icon = Frame::new(0,0,icon_w,h, None);
        let mut search_icon = SvgImage::load("search-24px.svg").unwrap();
        icon.draw(move |f|{
            search_icon.draw(f.x(), f.y(), 24, 24);    
        });
        icon.set_frame(FrameType::NoBox); 
        Icon{
            frame: icon
        }
    }
}

impl Deref for Icon {
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        &self.frame
    }
}

impl DerefMut for Icon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frame
    }
}
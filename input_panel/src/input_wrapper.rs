use fltk::{app, draw::*, frame::*, input::*, group::*, button::*};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::{FltkInput, CustomInput};


#[derive(Clone)]
pub struct InputWrapper{
    pub pack: Pack,
    pub inner_group: Group,
    pub label_frame: Frame,
    pub input: CustomInput,
    pub name: String, 
}


impl InputWrapper {
    pub fn new(input: CustomInput, name: String) -> Self {

        let mut pack = Pack::new(0,0,100, 25, "");
        pack.end();

        let mut inner_group = Group::new(0,0,100, 25, "");
        inner_group.end();

        let mut label_frame = Frame::new(0,0,25,25,"");
        label_frame.set_align(Align::Inside | Align::Right);
        label_frame.set_frame(FrameType::FlatBox);
        label_frame.set_label_color(Color::White);
        label_frame.set_color(fltk::enums::Color::Dark3);
        let name_c = name.clone();
        label_frame.draw2(move|widg|{
            set_draw_color(Color::White);
            draw_text2(&name_c, widg.x()-5, widg.y(), widg.width(), widg.height(), Align::Inside | Align::Right);
        });

        let mut divider = Frame::new(0,0,25,5,"");
        divider.set_frame(FrameType::FlatBox);
        divider.set_color(Color::Dark3);

        inner_group.add(&label_frame);
        inner_group.add(&input.pack());

        pack.add(&divider);
        pack.add(&inner_group);

        let mut o = InputWrapper {
            pack: pack, 
            inner_group: inner_group,
            label_frame: label_frame,
            input: input,
            name: name, 
        };
        
        o
    }

    
}

impl Deref for InputWrapper {
    type Target = Group;

    fn deref(&self) -> &Self::Target {
        &self.inner_group
    }
}

impl DerefMut for InputWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner_group
    }
}

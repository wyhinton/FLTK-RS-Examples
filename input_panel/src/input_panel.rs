use super::InputWrapper;
use crate::CustomInputValue;
use fltk::{enums::*, frame::Frame, group::Group, group::Pack, prelude::*};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct InputPanel<'a> {
    pub pack: Pack,
    pub padding_pack: Group,
    pub inputs: Vec<&'a InputWrapper>,
    pub header: Frame,
}

impl<'a> InputPanel<'a> {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: String) -> Self {
        let mut test_group = Group::new(x, y, w, h, "");
        test_group.end();
        test_group.set_frame(FrameType::FlatBox);
        test_group.set_color(Color::Dark3);

        let mut pack = Pack::new(x, y, w, h, "");
        pack.end();

        let mut header = Frame::default()
            .with_pos(0, 0)
            .with_size(200, 25)
            .with_label(&label);
        test_group.add(&header);

        test_group.add(&pack);
        pack.resize(
            test_group.x() + 15,
            test_group.y() + 20,
            test_group.width() - 30,
            test_group.height() - 5,
        );

        let mut sb = InputPanel {
            pack: pack,
            padding_pack: test_group,
            header: header,
            inputs: vec![],
        };

        sb.header.set_frame(FrameType::FlatBox);
        sb.header.set_label_color(Color::White);
        sb.header.set_align(Align::Inside | Align::Left);
        sb.header.set_color(Color::from_rgb(64, 64, 64));

        sb
    }
    pub fn add_input(&mut self, input_wrapper: &'a InputWrapper) {
        self.pack.add(&input_wrapper.pack);
        self.padding_pack.resize(
            self.padding_pack.x(),
            self.padding_pack.y(),
            self.padding_pack.width(),
            self.padding_pack.height() + &input_wrapper.height(),
        );
        self.inputs.push(input_wrapper);
    }
    // pub fn get_values(&self){
    pub fn get_values(&self) -> HashMap<String, CustomInputValue> {
        dbg!("getting all values for input");
        let mut my_vals = vec![];

        for x in &self.inputs {
            let my_input_info = (x.name.clone(), x.input.value());
            my_vals.push(my_input_info);
        }

        dbg!(my_vals.clone());
        let panel_vals: HashMap<String, CustomInputValue> = my_vals.iter().cloned().collect();
        panel_vals
    }
}

impl<'a> Deref for InputPanel<'a> {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl<'a> DerefMut for InputPanel<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}

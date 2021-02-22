use fltk::{app, group::*, input::*, frame::*, button::*, draw::*};
use std::ops::{Deref, DerefMut};
use crate::{CustomInputValue, CustomInputExt, Error, Result, IncorrectInputType, FloatInputSettings, NumInputSettings, Distance};
use std::fmt;
use snafu::{ensure, Backtrace, Snafu};
// use syn::*;
use super::Message;
use std::str::FromStr;


// use std::s
// use super::Incc

#[derive(Clone)]
pub struct CBoolInput {
    pub pack: Pack, 
    pub input: CheckButton,
    // pub inp: Aligned
}


impl CustomInputExt for CBoolInput{
    fn value(&self)->CustomInputValue{
        CustomInputValue::CiBool(self.input.is_checked())
    }
    fn hide(&mut self){
        self.pack.hide();
    }
    // fn set_text_color(&mut self, text_color: fltk::enums::Color){
    //     //Do Nothing
    //     // self.input.set_text_color(fltk::enums::Color::Red);
    // }
    fn set_color(&mut self, color: fltk::enums::Color){
        self.input.set_color(color);
    }
    fn set_value(&mut self, val: CustomInputValue )->Result<()>{

        match val {
            CustomInputValue::CiFloat(inp) => inp,
            _ => IncorrectInputType { kind: val }.fail()?
              // ^^^^^^^^^^^^^^^^^^ no Error::
       };
        Ok(())
    }
    fn pack(&self)->Pack{
        self.pack.clone()
    }
    fn redraw_input(&mut self){
        self.input.redraw()
    }
}
// fn foo() -> Result<(), MyError> {
//     WidgetNotFound { widget_name: "Quux" }.fail()
// }
impl fmt::Debug for CBoolInput{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "hello i'm a bool input")
    }
}

impl CBoolInput {
    pub fn new()-> Self {
        let mut pack = Pack::new(25,0,75,25,"");
        pack.end();
        
        let mut input = CheckButton::new(0,0,25,25,"");
        input.set_color(Color::Dark3);
        input.set_frame(FrameType::FlatBox);
        // pack.add(&spacer_frame);
        pack.add(&input);

        let mut fi = CBoolInput{
            pack: pack, 
            input: input,
        };

        fi
    }
}


impl Deref for CBoolInput {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for CBoolInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}
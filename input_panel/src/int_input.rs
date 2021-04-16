use fltk::{app, group::*, input::*, frame::*, button::*, draw::*};
use std::ops::{Deref, DerefMut};
use crate::{CustomInputValue, CustomInputExt, Error, Result, IncorrectInputType, NumInputSettings, Distance};
use std::fmt;
use snafu::{ensure, Backtrace, Snafu};
// use syn::*;
use super::Message;
use std::str::FromStr;


// use std::s
// use super::Incc

#[derive(Clone)]
pub struct CIntInput {
    pub pack: Pack, 
    pub input: Input,
    pub settings: NumInputSettings<i32>,
    pub display: Frame, 
}
#[derive(Clone, Debug)]
pub struct InputInfo {
    pub valid_input: i32,
    pub cur_value: String,
}

impl fmt::Debug for CIntInput{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "hello i'm a float input")
    }
}

impl CustomInputExt for CIntInput{
    fn value(&self)->CustomInputValue{
        let val = self.input.value().parse::<i32>().unwrap();
        CustomInputValue::CiInt(val)
    }
    fn hide(&mut self){
        self.pack.hide();
    }
    // fn set_text_color(&mut self, text_color: fltk::enums::Color){
    //     self.input.set_text_color(fltk::enums::Color::Red);
    // }
    fn redraw_input(&mut self){
        self.input.redraw()
    }
    fn set_color(&mut self, color: fltk::enums::Color){
        self.input.set_color(color);
    }
    fn set_value(&mut self, val: CustomInputValue )->Result<()>{

        match val {
            CustomInputValue::CiFloat(inp) => inp,
            _ => IncorrectInputType { kind: val }.fail()?
       };
        Ok(())
    }
    fn pack(&self)->Pack{
        self.pack.clone()
    }
}
// fn foo() -> Result<(), MyError> {
//     WidgetNotFound { widget_name: "Quux" }.fail()
// }

impl CIntInput {
    pub fn new(int_input_settings: NumInputSettings::<i32>)-> Self {
        
        let mut pack = Pack::new(25,0,75,25,"");
        pack.end();
        pack.set_frame(FrameType::BorderFrame);
        pack.set_color(Color::Black);

        let mut input = Input::new(0,0,75,25,"");
        input.set_color(Color::Dark2);
        input.set_frame(FrameType::FlatBox);

        input.set_value(&int_input_settings.default.to_string());
        input.set_text_color(Color::White);
        pack.add(&input);
        pack.set_color(Color::Blue);

        let def_val = int_input_settings.default;

        pack.add(&Button::default());
        input.hide();

        let mut display_frame = Frame::new(0,0,25,25, &int_input_settings.default.to_string());
        display_frame.set_label_color(Color::White);
        display_frame.set_frame(FrameType::FlatBox);
        display_frame.set_color(Color::Dark3);
        pack.add(&display_frame);
        input.hide();


        let mut fi = CIntInput{
            pack: pack, 
            input: input,
            settings: int_input_settings,
            display: display_frame,
        };

        let mut inp_c = fi.input.clone();
        let mut dis_c = fi.display.clone();
        let settings_c = fi.settings.clone();

        fi.pack.handle2(move |t, ev| match ev {
            // test_info = InputInfo{
            //     cur_value = t.value()
            // }
            Event::Enter=>{
                dis_c.set_color(fltk::enums::Color::Dark3);
                dis_c.redraw();
                // t.draw(move||{});
                true   
            }
            Event::Push => {
                println!("{}", "clicked group");
                dis_c.hide();
                inp_c.show();
                inp_c.take_focus();
                true
            },
            //lighten on hover
            Event::Move =>{
                // g2.redraw();
                true
            },
            Event::Leave =>{
                t.set_color(fltk::enums::Color::Dark2);
                t.redraw();
                println!("{}", "mouse leave group");
                true
            },

            _ => false,
        });
            
        let mut inp_c2 = fi.input.clone();
        let mut disp_c2 = fi.display.clone();

        fi.input.handle2(move |t, ev| match ev {
            Event::Unfocus=>{
                inp_c2.hide();
                disp_c2.show();
                disp_c2.redraw();
                let val = inp_c2.value();
                
                let mut final_val_float: f64 = match val.parse(){
                    Ok(num)=>num,
                    Err(e)=>panic!("{:?}",e)
                };

                final_val_float = checked_set_value(final_val_float, settings_c);
                

                let final_val_int = final_val_float as i32;
                inp_c2.set_value(&final_val_int.to_string());
                dbg!(final_val_int);
                disp_c2.set_label(&final_val_int.to_string());
                true
            },
            Event::KeyUp => {
                match app::event_key() {
                        Key::Up=>{
                            println!("{}", "got key up");
                 
                            let up_val = increment(&mut disp_c2, 1, settings_c);
                            disp_c2.redraw();
                            dbg!(up_val);
                            t.set_value(&up_val.to_string());
                        }
                        Key::Down=>{
                            println!("{}", "got key up");
                            let down_val = decrement(&mut disp_c2, 1, settings_c);
                            disp_c2.redraw();
                            t.set_value(&down_val.to_string());
                        }
                        Key::Tab=>{
                            dbg!("got tab key at float input");
                            //SWITCH THE NEXT INPUT
                            t.parent().unwrap().do_callback();
                        }
                        _=>()
                    }
                true
            }
    
            _ => false,
        });

        
            
        fi
    }

}

fn checked_set_value(val: f64, settings: NumInputSettings<i32>)->f64{
    if val > settings.max as f64{
        return settings.max as f64
    }
    if val < settings.min as f64{
        return settings.min as f64
    }
    val
}

fn increment(display_frame: &mut Frame, increment: i32, settings: NumInputSettings<i32>)->i32{
    let original_value = display_frame.label().parse::<i32>().unwrap();
    let mut cur_value = display_frame.label().parse::<i32>().unwrap();
    
    if app::is_event_shift(){
        cur_value = cur_value + increment*10;
    } else if app::is_event_alt(){
        cur_value = cur_value + increment;
    } else {
        cur_value = cur_value + increment;
    }
    if cur_value < settings.max+1{
        display_frame.set_label(&cur_value.to_string());
        return cur_value
    } else {
        display_frame.set_label(&original_value.to_string());
        return original_value
    }
    return cur_value
}

fn decrement(display_frame: &mut Frame, increment: i32, settings: NumInputSettings<i32>)->i32{
    let original_value = display_frame.label().parse::<i32>().unwrap();
    let mut cur_value = display_frame.label().parse::<i32>().unwrap();
    
    if app::is_event_shift(){
        cur_value = cur_value - increment*10;
    } else if app::is_event_alt(){
        cur_value = cur_value - increment*1;
    } else {
        cur_value = cur_value - increment;
    }
    if cur_value > settings.min - 1{
        display_frame.set_label(&cur_value.to_string());
        return cur_value
    } else {
        display_frame.set_label(&original_value.to_string());
        return original_value
    }
}

fn format_float_from_input(inp: &str)->String{
    let mut inps = String::from(inp);
    if !inps.contains("."){
        inps = inps + &".00".to_string();
    }
    inps
}


fn draw_centered(input: &mut Input, val: i32){
    input.draw2(move |widg| {
        let x = widg.x(); 
        let y = widg.y();
        let width = widg.width();
        let height = widg.height();
        push_clip(x, y, width, height);
        draw_box(FrameType::NoBox, x, y, width, height, Color::Black);
        // draw_box(FrameType::DownBox, x, y, width, height, Color::White);
        set_draw_color(Color::Black);
        draw_text2(&val.to_string(), x, y, width, height, Align::Center);
        pop_clip();
    });
}

impl Deref for CIntInput {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for CIntInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}
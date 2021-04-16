//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*};
use rand::{distributions::Alphanumeric, Rng};
use snafu::{Backtrace, ResultExt, Snafu, ensure};
pub mod enums;
use enums::*;
use std::fmt;
use dyn_clone::DynClone;

pub type FltkInput = Box<dyn fltk::prelude::ValuatorExt + Send + Sync>;
pub type CustomInput = Box<dyn  CustomInputExt + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;


pub mod input_wrapper; 
use input_wrapper::InputWrapper; 
pub mod float_input;
use float_input::CFloatInput;
pub mod input_panel;
pub mod int_input;  
use int_input::CIntInput;
use input_panel::InputPanel;
pub mod bool_input;
use bool_input::CBoolInput;

#[derive(Debug, Snafu)]
pub enum Error{
    #[snafu(display("Incorrect Type: {:?}", kind))]
    IncorrectInputType{kind: CustomInputValue},
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Distance {
    integral: u64,
    fractional: u64
}

impl Distance {
    fn new(i: u64, f: u64) -> Distance {
        Distance {
            integral: i,
            fractional: f
        }
    }
    fn decode(&self)->f64{
        let combined_string_arr = vec![self.integral.to_string(), ".".to_string(), self.fractional.to_string()];
        let combined_string = combined_string_arr.join("");
        let val = combined_string.parse::<f64>().unwrap();
        val
    }
}

pub trait CustomInputExt:DynClone + fmt::Debug{
    fn value(&self)->CustomInputValue;
    fn hide(&mut self); 
    fn set_color(&mut self, color: fltk::enums::Color);    
    fn set_value(&mut self, val: CustomInputValue )->Result<()>;
    fn pack(&self)->Pack;
    fn redraw_input(&mut self);
}

dyn_clone::clone_trait_object!(CustomInputExt);
#[derive(fmt::Debug, Clone, Copy)]
pub enum Message {
    Test,   
    GetInputValues,
}

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    win.make_resizable(true);

    let panel_width = 300;
    let panel_x =  win.width()/2 - panel_width/2;
    
    let mut my_panel = InputPanel::new(panel_x, 300, panel_width, 100, "My Inputs".to_string());
    let first_input = InputWrapper::new(Box::new(CFloatInput::new(NumInputSettings::<f64>{min: 0.0, max: 100.0, default: 50.0, step: 1.0})), "Float Input".to_string());
    let second_float = InputWrapper::new(Box::new(CFloatInput::new(NumInputSettings::<f64>{min: 0.0, max: 1.0, default: 0.50, step: 0.1})), "Float Input".to_string());
    let second_input = InputWrapper::new(Box::new(CIntInput::new(NumInputSettings::<i32>{min: 0, max: 1000, default: 50, step: 1})), "Int Input".to_string());
    let bool_input= InputWrapper::new(Box::new(CBoolInput::new()), "Bool Input".to_string());

    my_panel.add_input(&bool_input);
    my_panel.add_input(&first_input);
    my_panel.add_input(&second_float);
    my_panel.add_input(&second_input);

    let mut get_inputs_button= Button::new(my_panel.x(),my_panel.y()+300,200,30, "get inputs");
    get_inputs_button.emit(s, Message::GetInputValues);
    win.end();
    win.show();

    // app::set_callback(widget, cb)
    // app
    app::add_timeout(1.0, move||{app::redraw()});

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    println!("{}", "got test message");
                    app::redraw();
                }
               GetInputValues=>{
                   println!("{}", "got input vlaues");
                   let panel_vals = my_panel.get_values();
                   let test_float_val = panel_vals.get("Float Input").unwrap();
                   dbg!(panel_vals);
               }
            }
        }
    }
}

// fn integer_decode(val: f64) -> (u64, i16, i8) {
//     let bits: u64 = unsafe { mem::transmute(val) };
//     let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
//     let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
//     let mantissa = if exponent == 0 {
//         (bits & 0xfffffffffffff) << 1
//     } else {
//         (bits & 0xfffffffffffff) | 0x10000000000000
//     };

//     exponent -= 1023 + 52;
//     (mantissa, exponent, sign)
// }
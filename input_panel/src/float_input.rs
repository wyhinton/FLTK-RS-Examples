use fltk::{app, group::*, input::*, frame::*, button::*, draw::*, valuator::*, };
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
pub struct CFloatInput {
    pub pack: Pack, 
    pub input: ValueInput,
    // pub input: Input,
    pub settings: NumInputSettings<f64>,
    pub display: Frame, 
    // pub inp: Aligned
}
#[derive(Clone, Debug)]
pub struct InputInfo {
    pub valid_input: f64,
    pub cur_value: String,
}

impl fmt::Debug for CFloatInput{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "hello i'm a float input")
    }
}
impl CustomInputExt for CFloatInput{
    fn value(&self)->CustomInputValue{
        // CustomInputValue::CiFloat(self.input.value().parse::<f64>().unwrap())
        // let val = self.input.value().parse::<f64>().unwrap();
        let val = self.input.value();
        dbg!(val);
        let fractional = get_frac(val);
        dbg!(fractional);
        let integral = val as u64;
        dbg!(integral);
        let typed_val = CustomInputValue::CiFloat(Distance::new(integral, fractional));
        
        typed_val
        // CustomInputValue::CiFloat(self.input.value().parse::<f64>().unwrap())
        // CustomInputValue::CiFloat(self.input.value().parse::<f64>().unwrap())
    }
    fn hide(&mut self){
        self.pack.hide();
    }
    // fn set_text_color(&mut self, text_color: fltk::enums::Color){
    //     self.input.set_text_color(fltk::enums::Color::Red);
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

impl CFloatInput {
    pub fn new(float_settings: NumInputSettings<f64>)-> Self {
        
        let mut pack = Pack::new(25,0,75,25,"");
        pack.end();
        pack.set_frame(FrameType::BorderFrame);
        pack.set_color(Color::Black);

        let mut input = ValueInput::new(0,0,75,25,"");
        input.set_bounds(float_settings.min, float_settings.max);

        input.set_maximum(float_settings.max);
        input.set_color(Color::Dark2);
        input.set_frame(FrameType::FlatBox);
        // input.set_text_color(Color::Red);
        // ValueInput::set_text_color(input, Color::Red);
        // dbg!(input.text_color());

        input.set_value(float_settings.default);
        pack.add(&input);
        pack.set_color(Color::Blue);

        let def_val = float_settings.default;

        pack.add(&Button::default());
        input.hide();

        let mut display_frame = Frame::new(0,0,200,25, &float_settings.default.to_string());
        display_frame.set_label_color(Color::White);
        display_frame.set_frame(FrameType::FlatBox);
        display_frame.set_color(Color::Dark3);
        pack.add(&display_frame);
        input.hide();

        dbg!(float_settings.default);
        draw_fill(&mut display_frame, float_settings.default, float_settings);
        // display_frame.redraw();

        let mut fi = CFloatInput{
            pack: pack, 
            input: input,
            settings: float_settings,
            display: display_frame,
        };

        let mut inp_c = fi.input.clone();
        let mut dis_c = fi.display.clone();

        fi.pack.handle2(move |t, ev| match ev {
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
        let settings_c = fi.settings.clone();
        
    
        //UP DOWN INCREMENTING
        fi.input.handle2(move |t, ev| match ev {
            Event::Unfocus=>{
                inp_c2.hide();
                disp_c2.show();
                disp_c2.redraw();
                
                let val = inp_c2.value();
                let final_val: f64 = checked_set_value(inp_c2.value(), settings_c);
                let formatted_val = convert(final_val, 2);
                let test_str = formatted_val.clone();
                
                inp_c2.set_value(final_val);
                draw_fill(&mut disp_c2, final_val, settings_c);
                disp_c2.redraw();
                true
            },
            Event::KeyUp => {
                match app::event_key() {
                        Key::Up=>{
                            println!("{}", "got key up");
                            let up_val = increment(&mut disp_c2, settings_c.step, settings_c);
                            disp_c2.redraw();
                            dbg!(up_val);
               
                            t.set_value(up_val);
                        }
                        Key::Down=>{
                            println!("{}", "got key up");
                            let down_val = decrement(&mut disp_c2, settings_c.step, settings_c);
                            disp_c2.redraw();
                            t.set_value(down_val);
                            // t.set_value(&convert(down_val, 2));
                        }
                        Key::Tab=>{
                            dbg!("got tab key at float input");
                            //SWITCH THE NEXT INPUT
                            // t.parent().unwrap().do_callback();
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

fn convert(val: f64, precision: usize) -> String {
    format!("{:.prec$}", val, prec = precision)
}

fn checked_set_value(val: f64, settings: NumInputSettings<f64>)->f64{
    if val > settings.max{
        return settings.max
    }
    if val < settings.min{
        return settings.min
    }
    val
}
fn get_frac(f: f64) -> u64 {
    
    let eps = 1e-4;
    let mut f = f.abs().fract();
    if f == 0.0 { return 0; }
    
    while (f.round() - f).abs() <= eps {
        f = 10.0 * f;
    }
    
    while (f.round() - f).abs() > eps {
        f = 10.0 * f;
    }
    
    return f.round() as u64;
}

fn increment(display_frame: &mut Frame, increment: f64, settings: NumInputSettings<f64>)->f64{
    let original_value = display_frame.label().parse::<f64>().unwrap();
    let mut cur_value = display_frame.label().parse::<f64>().unwrap();

    if app::is_event_shift(){
        cur_value = cur_value + increment*10.0;
    } else if app::is_event_alt(){
        cur_value = cur_value + increment*0.1;
    } else {
        cur_value = cur_value + increment;
    }
    if cur_value < settings.max{
        display_frame.set_label(&cur_value.to_string());
        return cur_value
    } else {
        display_frame.set_label(&original_value.to_string());
        return original_value
    }

}

fn decrement(display_frame: &mut Frame, increment: f64, settings: NumInputSettings<f64>)->f64{
    dbg!(display_frame.label());
    let original_value = display_frame.label().parse::<f64>().unwrap();
    let mut cur_value = display_frame.label().parse::<f64>().unwrap();
    cur_value = (cur_value*100.0).round()/100.0;
    // let y = (x * 100.0).round() / 100.0;
    dbg!(cur_value);
    
    if app::is_event_shift() && cur_value > settings.min {
        cur_value = cur_value - increment*10.0;
        display_frame.set_label(&cur_value.to_string());
        return cur_value
    } 
    if app::is_event_alt() && cur_value > settings.min {
        cur_value = cur_value - increment*0.1;
        display_frame.set_label(&cur_value.to_string());
        return cur_value
    } 
    if cur_value > settings.min {
        cur_value = cur_value - increment;
        display_frame.set_label(&cur_value.to_string());
        return cur_value
    }
    original_value

}

fn format_float_from_input(inp: &str)->String{
    let mut inps = String::from(inp);
    if !inps.contains("."){
        inps = inps + &".00".to_string();
    }
    inps
}

fn draw_centered(input: &mut Input, val: f64){
    input.draw2(move |widg| {
        let x = widg.x(); 
        let y = widg.y();
        let width = widg.width();
        let height = widg.height();
        push_clip(x, y, width, height);
        draw_box(FrameType::NoBox, x, y, width, height, Color::Black);
        set_draw_color(Color::Black);
        draw_text2(&val.to_string(), x, y, width, height, Align::Center);
        pop_clip();
    });
}

fn draw_fill(frame: &mut Frame, val: f64, settings: NumInputSettings<f64>){
    dbg!(val);
    dbg!(frame.width());
    dbg!(val/settings.max);
    let percentage =((val/settings.max) * frame.width() as f64) as i32;
    dbg!(percentage);
    let draw_val =convert(val, 2);
    dbg!(draw_val.clone());

    frame.draw2(move|widg|{
        draw_box(
            FrameType::GtkDownBox,
            widg.x(),
            widg.y(),
            percentage, 
            widg.height(),
            Color::Blue,
        );
        draw_box(
       
            FrameType::FlatBox,
            widg.x()+percentage,
            widg.y(),
            widg.width()-percentage, 
            widg.height(),
            Color::from_rgb(62, 62, 62),
        );
        set_draw_color(Color::White);
        draw_text2(&draw_val, widg.x(), widg.y(), widg.width(), widg.height(), Align::Inside | Align::Center);
    })

}

impl Deref for CFloatInput {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for CFloatInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}
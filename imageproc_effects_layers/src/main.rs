//Demonstrates Photoshop style adjustment layers

use fltk::{app, app::*, frame::*, valuator::*, window::*, widget::*,  image::RgbImage, table::*, button::*, input::*, group::*};
// pub mod effect_configurations; 
// use effect_configurations::*; 
use image::GenericImageView;
use image::imageops::*; 
use std::{default::Default, vec}; 
mod dragable_container; 
use dragable_container::DragableContainer;
use dragable_container::drag_item::DragItem;
use uuid::Uuid; 
use strum::EnumMessage;
use strum_macros::*;
use strum_macros::*; 
use strum_macros::EnumIter;
use strum::IntoEnumIterator; 
use strum_macros::EnumMessage;

mod processing_functions; 
// use processing_functions;


use std::fmt;
use field_types::FieldType;
pub mod icon_toggle_button;
use icon_toggle_button::IconToggleButton;
#[derive(Clone, EnumIter, EnumString, EnumMessage)]
// #[derive(Clone, EnumIter, EnumString, EnumMessage)]
pub enum Operation{
    #[strum(message = "Add")]
    Add{
        config: AddConfig
    },
    #[strum(message ="Test")]
    TestOperation{
        config: TestConfig,
    }
}

impl fmt::Debug for Operation{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::Add{..} => write!(f, "Add Op"),
            Operation::TestOperation{..} => write!(f, "Test Op"),
        }
     }
}


// pub mod::
#[derive(Clone)]
pub enum Message{
    Test,   
    ProcessLayers,
    SetProcessLayers(Vec<DragItem>),
    AddOperation(Operation),
    RemoveOperation(Uuid),
    DeactivateOperation(Uuid),
    HideOperation(Uuid),
    UpdateInputValue(Uuid, Uuid, f64),
}

pub trait OpConfigExt {
    fn as_array(&self)->Vec<ConfigVal>;
    // fn 
}

//CONFIG VAL
#[derive(Clone)]
pub struct ConfigVal{
    pub id: Uuid,
    pub val_name: String, 
    pub val: f64, 
    pub inp_type: SliderType,
    pub bounds: (f64, f64),
    pub step: f64,
}

impl ConfigVal{
    pub fn get_value(&self) -> f64{
        self.val
    }
    pub fn set_value(&mut self, new_val: f64){
        self.val = new_val; 
    }
}

pub struct OpLayer{
    op: Operation, 
    configs: Vec<ConfigVal>
}

impl fmt::Debug for OpLayer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "operation: {:?}, inputs: {:?}", self.op, self.configs)
    }
}


impl fmt::Debug for ConfigVal{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value Name: {}, value: {}, Input Type: {:?}, bounds: {},{} step: {} ", self.val_name, self.val, self.inp_type, self.bounds.0, self.bounds.1, self.step)
    }
}
//TEST CONFIG
#[derive(Clone)]//input struct
pub struct TestConfig{
    pub val_a: ConfigVal,
    pub val_b: ConfigVal, 
    pub inputs: Vec<ConfigVal>,
}

impl Default for TestConfig{
    fn default() -> TestConfig{
        TestConfig {
            val_a: ConfigVal{
                id: Uuid::new_v4(),
                val_name: "My Test Value 1".to_string(),
                val: 0.0,
                inp_type: SliderType::HorizontalFill,
                // inp_type: ProcessInputType::HorizontalFill,
                bounds: (0.0, 255.0),
                step: 1.0,
            },
            val_b: ConfigVal{
                id: Uuid::new_v4(),
                val_name: "My Test Value 1".to_string(),
                val: 255.0,
                inp_type: SliderType::Horizontal,
                // inp_type: ProcessInputType::Horizontal,
                bounds: (0.0, 255.0),
                step: 1.0,
            },
            inputs: vec![
                ConfigVal{
                    id: Uuid::new_v4(),
                    val_name: "My Test Value 1".to_string(),
                    val: 255.0,
                    inp_type: SliderType::Horizontal,
                    // inp_type: ProcessInputType::Horizontal,
                    bounds: (0.0, 255.0),
                    step: 1.0,
                }, 
                ConfigVal{
                    id: Uuid::new_v4(),
                    val_name: "My Test Value 1".to_string(),
                    val: 0.0,
                    inp_type: SliderType::HorizontalFill,
                    // inp_type: ProcessInputType::HorizontalFill,
                    bounds: (0.0, 255.0),
                    step: 1.0,
                }
            ]
        }

    }
}

impl OpConfigExt for TestConfig{
    fn as_array(&self) -> Vec<ConfigVal> {
        [self.val_a.clone(), self.val_b.clone()].to_vec()
    }
}

impl fmt::Debug for TestConfig{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "val_a: {:?}, val_b: {:?}", self.val_a, self.val_b)
    }
}

//ADD CONFIG
#[derive(Debug, Clone)]
pub struct AddConfig{
    pub inputs: Vec<ConfigVal>,
    pub val_a: ConfigVal,
}

impl Default for AddConfig{
    fn default() -> AddConfig{
        AddConfig{
            inputs: vec![ConfigVal{
                id: Uuid::new_v4(),
                val_name: "Add".to_string(), 
                val: 5.0, 
                inp_type: SliderType::HorizontalFill,
                bounds: (0.0, 250.0),
                step: 1.0,
            }],
            val_a: ConfigVal{
                id: Uuid::new_v4(),
                val_name: "Add".to_string(), 
                val: 5.0, 
                inp_type: SliderType::HorizontalFill,
                bounds: (0.0, 250.0),
                step: 1.0,
            }
        }
    }
}



impl OpConfigExt for AddConfig{
    fn as_array(&self) -> Vec<ConfigVal> {
        [self.val_a.clone()].to_vec()
    }
}
//SUBTRACT CONFIG
#[derive(Debug, Clone)]
pub struct SubtractConfig{
    pub a: i32,
    pub b: i32,
}

impl Default for SubtractConfig{
    fn default() -> SubtractConfig{
        SubtractConfig{
            a: 0,
            b: 0,
        }
    }
}



fn center_img(wid: &Widget, mut rgb_img: RgbImage){
    rgb_img.draw(wid.x(), wid.y(), wid.width(), wid.height());
}


fn main() {

    // let mut my_funcs = Vec<fn(i32, i32) -> i32 = vec[add(1, 2), substra]
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    win.make_resizable(true);

    let mut tb = Button::new((win.width()/2)-50, 800, 100, 50, "");

    let mut test_img_frame = Frame::new(20, 20, 300, 300, "");
    let img = image::open("imgs/clouds.jpeg").unwrap();
    let contrasted = image::imageops::colorops::contrast(&img, 100.0).into_raw(); 
    // let contrasted = image::imageops::colorops::contrast(&img, 100.0).to_image(); 
    // let contrasted = image::imageops::colorops::contrast(&img, 100.0).to_image().to_vec(); 
    let (x, y) = img.dimensions();
    let rgb = RgbImage::new(&contrasted, x, y, 4).unwrap();
    // let rgb = RgbImage::new(&img.to_bytes(), x, y, 3).unwrap();
    test_img_frame.set_image(Some(rgb));

    let sc1 = s.clone();
    tb.set_callback(move||{
        sc1.send(Message::ProcessLayers);
    });
    let dcw = 300; 
    let dch = 500;
    let dcx = (win.width()/2) - (dcw/2);
    let dcy = 100;
    let mut cont = DragableContainer::new(dcx, dcy, dcw, dch, s.clone());

    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    // println!("{}", "got test message");
                    app::redraw();
                    cont.redraw();
                }
                //DRAGGABLE ITEMS
                ProcessLayers => {
                    let co = cont.get_ops();
                    println!("OPS AT PROCESS LAYERS: {:?}", co);
                    println!("num operations: {}", co.len());
                    let result = pipe_function_array(0.0, co);
                    println!("result is:  {}", result);
                }
                SetProcessLayers(layers) => {
                    cont.set_ops(layers);    
                }
                AddOperation(new_op)=>{
                    cont.add_op(new_op, s.clone());
                    // dbg!(new_op);
                }
                RemoveOperation(drag_item_to_remove) => {
                    println!("should remove at main: {:?}", drag_item_to_remove);
                    cont.delete_op(drag_item_to_remove, s.clone());
                    
                }
                HideOperation(drag_item_to_hide_id) => {
                    println!("should hide at main: {:?}", drag_item_to_hide_id);
                    cont.hide_op(drag_item_to_hide_id, s.clone());
                    
                }
                DeactivateOperation(drag_item_to_deactivate_id) => {
                    println!("should hide at main: {:?}", drag_item_to_deactivate_id);
                    cont.set_inactive(drag_item_to_deactivate_id, s.clone());
                }
                
                //INPUT FIELDS
                UpdateInputValue(drag_item_id, input_id_to_update, input_val) => {
                    cont.update_item_input(drag_item_id, input_id_to_update, input_val); 
                }
            }
        }
    }
}
//pipe function takes in an array of config value arrays, executing them sequentiallty and piping the results along the way
fn pipe_function_array(mut initial_value: f64, a: Vec<OpLayer>) -> f64 {
    a.iter()
        .fold(initial_value, |val, operation, | match operation.op {
            Operation::Add {..} => processing_functions::add(val, operation.configs.clone()),
            Operation::TestOperation {..} => processing_functions::test_op(val, operation.configs.clone()),
        })
}


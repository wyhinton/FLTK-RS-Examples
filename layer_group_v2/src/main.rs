#![warn(clippy::all, rust_2018_idioms)]
use fltk::{app, app::*, group::Pack, image::*, group::*, self, button::*, draw, enums::*, frame::*, draw::*, prelude::*, widget::*, window::*};
use std::{convert::TryInto, ops::{Deref, DerefMut}};
use comp_state::{topo, use_state, use_state_current, StateAccess, CloneState};
use uuid::Uuid;
pub mod utils;
use event_emitter_rs::EventEmitter;
pub mod custom_emmiter;
use custom_emmiter::CustomEmmiter;
pub mod enums;
use std::cell::RefCell;
use std::rc::Rc;
use enums::*;
use std::sync::Mutex;
mod dropable_manager;
use dropable_manager::DropableManager;
use state::{Storage, LocalStorage};
use std::collections::HashMap;
use crossbeam_channel::unbounded;
use lazy_static::lazy_static;
use std::time::Duration;
mod layer_container;
use layer_container::LayerContainer;
mod traits;
use traits::*;
mod droppable_button;
use droppable_button::DroppableButton;
use std::sync::Arc;
use multi_mut::{HashMapMultiMut, HashMapMutWrapper};
use colored::Colorize;
mod drag_layer;
use drag_layer::LayerDraggable;
#[macro_use]
mod macros; 
use macros::*;
pub struct DraggableEvent{}
impl DraggableEvent{
    pub const DRAGGABLE_PUSH: &'static str = "DRAGGABLE_PUSH";
}
type DraggableCb = fn(&mut Widget, Option<CustomEmmiter>, DraggableInfo);
type DragObject = Box<dyn DraggableExt>;
type DropObject = Box<dyn DroppableExt + Send>;


static ROOT_ID: &'static str = "__ROOT__";
static GLOBAL_MAP: Storage<Mutex<HashMap<String, WidgetValue>>> = Storage::new();
pub struct GlobalEvents{}

impl GlobalEvents{
    pub const COUNTER_UPDATE: i32 = 41;
}

lazy_static! {
    static ref DND_CHANNEL: Mutex<(crossbeam_channel::Sender<DragAction>, crossbeam_channel::Receiver<DragAction>)> = Mutex::new(unbounded::<DragAction>());
    static ref DROPPABLES: Mutex<HashMap<String, DropObject>> = Mutex::new(HashMap::new());
    static ref NEW_DROPPABLES: Mutex<HashMap<String, TestDroppable>> = Mutex::new(HashMap::new()) ;
}

#[derive(Clone, Debug)]
pub struct DragManager{
  pub source: Option<DraggableInfo>,
  pub target: Option<DraggableInfo>,
  pub under: Option<DraggableInfo>,
  pub drag_under: Option<DraggableInfo>,
}

impl DragManager{
    pub fn new()->Self{
        DragManager{
            source: None,
            target: None,
            under: None,
            drag_under: None,
        }
    }
}

pub struct DragResult{
    draggable_id: Uuid,
    drag_type: DraggableType,
    source: Box<dyn DroppableExt>
}

impl DragResult{
    pub fn new(draggable_id: Uuid, drag_type: DraggableType, source: Box<dyn DroppableExt>)->Self{
        DragResult{
            draggable_id: draggable_id,
            drag_type: drag_type,
            source: source
        }
    }
}

fn main() {
    let initial_map = HashMap::new();
    GLOBAL_MAP.set(Mutex::new(initial_map));

    let manager = Rc::new(RefCell::new(DragManager::new()));
    let app = App::default();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");

    let mut m_cont= Pack::new(200,200,600,600,None);
    m_cont.set_type(PackType::Horizontal);
    m_cont.set_spacing(20);
    // let mut test_container= TestDroppable::from(LayerContainer::new(300,300,300,600, "layer container 1"));
    // let test_convert: Result<LayerContainer, _> = test_container.try_into();
    // match test_convert{
    //     Ok(lc)=>{
    //         lc.say_hello()
    //     }
    //     Err(e)=>panic!(e)
    // }

    //layer container 1
    let mut container_1=LayerContainer::new(300,300,300,600, "layer container 1");
    container_1.add_new_layer("layer 1");
    container_1.add_new_layer("layer 2");
    container_1.add_new_layer("layer 3");
    container_1.add_new_layer("layer 4");
    NEW_DROPPABLES.lock().unwrap().insert(String::from("layer container 1"), TestDroppable::from(container_1));

    //droppable buttons
    let mut drop_trash = DroppableButton::new(50,50,"trash", "assets/delete_black_24dp.svg");
    NEW_DROPPABLES.lock().unwrap().insert(String::from("trash"), TestDroppable::from(drop_trash));
    let mut copy_button = DroppableButton::new(50,50,"trash", "assets/content_copy_black_24dp.svg");
    NEW_DROPPABLES.lock().unwrap().insert(String::from("trash"), TestDroppable::from(copy_button));
    
    //layer container 2
    let mut container_2= LayerContainer::new(700,300,300,600, "layer container 2");
    container_2.add_new_layer("layer 5");
    container_2.add_new_layer("layer 6");
    container_2.add_new_layer("layer 7");
    container_2.add_new_layer("layer 8");
    NEW_DROPPABLES.lock().unwrap().insert(String::from("layer container 2"), TestDroppable::from(container_2));
    
    m_cont.end();
    let (s,r) = DND_CHANNEL.lock().unwrap().clone();
    let manager_cl = manager.clone();
    let mut display_container= Pack::new(50,100,900,50,"Drag Manager");
    display_container.set_frame(FrameType::FlatBox);
    display_container.set_color(Color::White);

    let source_string = format!("{:?}", manager.borrow().source.clone());
    let mut source_display_frame=  DisplayFrame::new(0,0,900,20);
    source_display_frame.set_label(&source_string);
    
    // let under_string = format!("{:?}", manager.borrow().under.clone());
    // let mut under_display_frame= Frame::new(0,0,900,20, None);
    // under_display_frame.set_frame(FrameType::BorderFrame);
    // under_display_frame.set_color(Color::Blue);
    // under_display_frame.set_label(&under_string);
    
    let release_string = format!("{:?}", manager.borrow().under.clone());
    let mut target_display_frame= DisplayFrame::new(50,800,900,20);
    target_display_frame.set_label(&release_string);
    

    // display_container.set_damage_type(Damage::)
    display_container.draw(move |widg| {
        draw_rect_fill(widg.x(), widg.y(), widg.width(), widg.height(), Color::Red);
        for x in 0..widg.children(){
            widg.child(x).unwrap().set_damage(true);
        }
        // frame_cl.set_damage(true);
        widg.draw_children();
    });
    display_container.end();

    let mut droppables_display_frame= DisplayFrame::new(100,850,900,20);
    droppables_display_frame.set_label(&release_string);

    app::add_idle(move || {
        //using recv_timeout() is essential for saving CPU
        if let Ok(msg) = r.recv_timeout(Duration::from_millis(20)) {
            let source_string = format!("Source: {:?}", manager.borrow_mut().source.clone());
            source_display_frame.set_label(&source_string);
            match msg {
                DragAction::SetDragTarget(info) => {
                    // dbg!(format!("Setting Drag Target To: {}", info.draggable_id.clone()));
                    // dbg!(info.clone());
                    manager_cl.borrow_mut().target = Some(info.clone());
                    target_display_frame.redraw();
                    target_display_frame.set_damage(true);
                    target_display_frame.set_label(&format!("target: {:?}", info.clone()));
                }, 
                DragAction::SetDragSource(info)=>{
                    // dbg!("setting source");
                    manager_cl.borrow_mut().source = Some(info.clone());
                    source_display_frame.set_label(&format!("source: {:?}", info.clone()));
                    source_display_frame.redraw();
                    source_display_frame.set_damage(true);
                    source_display_frame.parent().unwrap().redraw();
                }
                DragAction::DragEnded=>{
                    // dbg!("drag eneded!");
                    // cdbg!(manager_cl.borrow());
                    // let test = NEW_DROPPABLES.lock().unwrap().

                    dbg!(NEW_DROPPABLES.lock().unwrap());
                    let dbg_string = format!("Droppables: {:?}", NEW_DROPPABLES.lock().unwrap());
                    // let test = dbg_string.split(",").flat_map(|f| format!("{}_GREAT", f)).collect();
                    droppables_display_frame.set_label(&dbg_string);
                    droppables_display_frame.redraw();
                    match (manager_cl.borrow().source.clone(), manager_cl.borrow().target.clone()){
                        (Some(source), Some(target))=>{
                            match ((source.draggable_type, target.draggable_type)){
                                ((DraggableType::Layer, DraggableType::Layer))=>{
                                    let source_layer_ref =  NEW_DROPPABLES.lock().unwrap().get(&source.draggable_id).unwrap().clone();
                                    let target_layer_ref =  NEW_DROPPABLES.lock().unwrap().get(&target.draggable_id).unwrap().clone();
                                    let source_layer_conversion: Result<LayerDraggable, _> = source_layer_ref.try_into();
                                    let target_layer_conversion: Result<LayerDraggable, _> = target_layer_ref.try_into();
                                    match ((source_layer_conversion, target_layer_conversion)){
                                        ((Ok(source_layer_cl), Ok(target_layer_cl)))=>{
                                            match NEW_DROPPABLES.lock().unwrap().get_pair_mut(&source_layer_cl.layer_parent_id(), &target_layer_cl.layer_parent_id()){
                                                Some((TestDroppable::LayerContainer(source_layer_container), TestDroppable::LayerContainer(target_layer_container)))=>{
                                                    dbg!(source_layer_container.droppable_id());
                                                    dbg!(target_layer_container.droppable_id());
                                                    dbg!(source_layer_cl.draggable_id());
                                                    let target_pos = target_layer_container.find_pos(target_layer_cl.draggable_id());
                                                    dbg!(target_pos);
                                                    let to_swap = source_layer_container.remove_child(source_layer_cl.draggable_id()).unwrap();
                                                    target_layer_container.insert_layer(to_swap.1, target_pos);
                                                }
                                                _=>()
                                            }
                                        }
                                        _=>()
                                    }
                                }
                                ((DraggableType::Layer, DraggableType::Container))=>{
                                    let source_layer_ref =  NEW_DROPPABLES.lock().unwrap().get(&source.draggable_id).unwrap().clone();
                                    // let target_layer_ref =  NEW_DROPPABLES.lock().unwrap().get(&target.draggable_id).unwrap().clone();
                                    let source_layer_conversion: Result<LayerDraggable, _> = source_layer_ref.try_into();
                                    // let target_layer_conversion: Result<LayerDraggable, _> = target_layer_ref.try_into();
                                    match (source_layer_conversion){
                                        Ok(source_layer_cl)=>{
                                            match NEW_DROPPABLES.lock().unwrap().get_pair_mut(&source_layer_cl.layer_parent_id(), &target.draggable_id){
                                                Some((TestDroppable::LayerContainer(source_layer_container), TestDroppable::LayerContainer(target_layer_container)))=>{
                                                    dbg!(source_layer_container.droppable_id());
                                                    dbg!(target_layer_container.droppable_id());
                                                    let to_swap = source_layer_container.remove_child(source_layer_cl.draggable_id()).unwrap();
                                                    target_layer_container.add_child(to_swap.1);
                                                }
                                                _=>()
                                            }
                                        }
                                        _=>()
                                    }
                                }
                                // (DraggableType::Layer, DraggableType::Container)=>{
                                //     match NEW_DROPPABLES.lock().unwrap().get_pair_mut(&source.draggable_id, &target.draggable_id){
                                //         Some((TestDroppable::Layer(source_layer), TestDroppable::LayerContainer(target_layer_container)))=>{
                                //             // dbg!(source_layer_container.droppable_id());
                                //             // dbg!(target_layer_container.droppable_id());
                                //             let to_swap = source_layer_container.remove_child(source_layer_cl.draggable_id()).unwrap();
                                //             target_layer_container.add_child(to_swap.1);
                                //         }
                                //         _=>()
                                //     }
                                //     // let target_layer_ref =  NEW_DROPPABLES.lock().unwrap().get(&target.draggable_id).unwrap().clone();
                                //     // let target_layer_conversion: Result<LayerDraggable, _> = target_layer_ref.try_into();
                                //     // match target_layer_conversion{
                                //     //     Ok(target_)
                                //     // }
                                // }
                                _=>()
                            }
                            // match NEW_DROPPABLES.lock().unwrap().get_pair(&source.draggable_id, &target.draggable_id){
                            // match NEW_DROPPABLES.lock().unwrap().get_pair_mut(&source.draggable_id, &target.draggable_id){
                            //     Some((TestDroppable::Layer(source_layer), TestDroppable::Layer(target_layer)))=>{
                            //         dbg!(source_layer.layer_widg().label());
                            //         dbg!(target_layer.layer_widg().label());
                            //         dbg!(source_layer.layer_parent_id());
                            //         dbg!(target_layer.layer_parent_id());

                            //         // let source_layer_ref =  NEW_DROPPABLES.try_lock().unwrap().get(&target.draggable_id).unwrap().clone();
                            //         // let target_layer_ref =  NEW_DROPPABLES.lock().unwrap().get(&source.draggable_id).unwrap().clone();
                            //         // let source_layer_conversion: Result<LayerDraggable, _> = target_layer_ref.try_into();
                            //         // let target_layer_conversion: Result<LayerDraggable, _> = source_layer_ref.try_into();
                            //         // match ((source_layer_conversion, target_layer_conversion)){
                            //         //     ((Ok(source_layer_cl), Ok(target_layer_cl)))=>{
                            //         //         // match NEW_DROPPABLES.lock().unwrap().get_pair_mut(&source_layer_cl.layer_parent_id(), &target_layer_cl.layer_parent_id()){
                            //         //         //     Some((source_parent, target_parent))=>{
                            //         //         //         dbg!(source_parent.droppable_id());
                            //         //         //         dbg!(target_parent.droppable_id());
                            //         //         //     }
                            //         //         //     _=>()
                            //         //         // }
                            //         //     }
                            //         //     _=>()
                            //         // }
                                    
                                    
                            //         // match ((source_parent_container_conversion, target_parent_container_conversion)){
                            //         //     (OK(source_container))
                            //         // }
                                    

                            //         // dbg!(NEW_DROPPABLES.lock().unwrap().get_pair_mut(&source.parent, &target.parent));
                            //         // match NEW_DROPPABLES.lock().unwrap().get_pair_mut(&source.parent, &target.parent){
                            //         //     Some((source_parent, target_parent))=>{
                            //         //         match ((source_parent, target_parent)){
                            //         //             (TestDroppable::LayerContainer(source_container), TestDroppable::LayerContainer(target_container))=>{
                            //         //                 dbg!("got both containers");
                            //         //                 let to_swap = source_container.remove_child(source.draggable_id).unwrap();
                            //         //                 target_container.add_child(to_swap.1);
                            //         //             }
                            //         //             _=>()
                            //         //         }
                            //         //         // let source_parent_container: Result<LayerContainer, _> = source_parent.try_into();
                            //         //         // let target_parent_container: Result<LayerContainer, _> = target_parent.try_into();
                            //         //         // let to_swap = source_parent_container.remove_child(source.draggable_id).unwrap();
                            //         //         // target_parent.add_child(to_swap.1);
                            //         //     }
                            //         //     None=>()
                            //         // }
                            //         // let to_swap = source_parent.remove_child(source.draggable_id).unwrap();
                            //         // target_parent.add_child(to_swap.1);
                            //     } 
                            //     // Some((LayerItem::Layer(source_layer), TestDroppable::LayerContainer(target_container)))=>{
                            //     //     dbg!(source_layer.index());
                            //     //     dbg!(target_container.droppable_id());
                            //     // }
                            //     _=>()
                            // }
                            //  match (source.draggable_type.clone(), target.draggable_type.clone()){
                                 //(Source, Target)
                                // if dragging a layer onto antoher layer, add it to the target layer's layer container
                                // (DraggableType::Layer, DraggableType::Layer)=>{
                                //     // dbg!("dragged from layer to layer");
                                //     match NEW_DROPPABLES.lock().unwrap().get_pair_mut(&source.parent, &target.parent){
                                //         Some((source_parent, target_parent))=>{
                                //             let to_swap = source_parent.remove_child(source.draggable_id).unwrap();
                                //             target_parent.add_child(to_swap.1);
                                //         }
                                //         None=>()
                                //     }
                                    
                                // },
                                // //if draggig a layer into a layer container (not onto a layer), then add it as the last child to that container
                                // (DraggableType::Layer, DraggableType::Container)=>{
                                //     cdbg!("dragged to container");
                                //     match NEW_DROPPABLES.lock().unwrap().get_pair_mut(&source.parent, &target.draggable_id){
                                //         Some((source_parent, target_parent))=>{
                                //             let to_swap = source_parent.remove_child(source.draggable_id).unwrap();
                                //             target_parent.add_child(to_swap.1);
                                //         }
                                //         None=>()
                                //     }
                                // }
                                // (DraggableType::Layer, DraggableType::Button)=>{
                                //     // dbg!("TARGET WAS BUTTON");
                                //     // let mut all =  NEW_DROPPABLES.lock().unwrap();
                                //     // dbg!("HELLO");
                                //     // dbg!(NEW_DROPPABLES.lock().unwrap().get(&target.draggable_id.clone()).unwrap().droppable_id());
                                //     // let mut pair = get_mut_pair(&mut NEW_DROPPABLES.lock().unwrap(), &source.parent.clone(), &target.draggable_id.clone());
                                //     // dbg!(&pair.0.droppable_id());
                                //     // let test = pair.1.on_drop(pair.0);
                                //     NEW_DROPPABLES.lock().unwrap().get_mut(&target.draggable_id).unwrap().on_drop(source.clone());
                                //     // dbg!("dragged from layer to button");
                                // }
                                // _=>({dbg!("it was something else");})
                            //  }
                        },
                        _=>()
                    }
                }

                _=>()
            }
        }
    });


    win.end();
    win.show();

    app.run().unwrap();
}

#[derive(Clone)]
pub struct DisplayFrame{
  frame: Frame
}

impl DisplayFrame{
    pub fn new(x: i32, y: i32, width: i32, height: i32)->Self{
    let mut frame= Frame::new(x,y,width,height, None);
    frame.set_frame(FrameType::FlatBox);
    frame.set_color(Color::White);
    DisplayFrame{
        frame: frame
    }
}
}
impl std::ops::Deref for DisplayFrame{
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        &self.frame
    }
}

impl std::ops::DerefMut for DisplayFrame{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frame
    }
}
// fn get_mut_pair<'a, K, V>(conns: &'a mut HashMap<K, V>, a: &K, b: &K) -> (&'a mut V, &'a mut V)
// where
//     K: Eq + std::hash::Hash,
// {
//     unsafe {
//         let a = conns.get_mut(a).unwrap() as *mut _;
//         let b = conns.get_mut(b).unwrap() as *mut _;
//         assert_ne!(a, b, "The two keys must not resolve to the same value");
//         (&mut *a, &mut *b)
//     }
// }


//     // container_1.add_child()
    // let mut container_1= TestDroppable::from(LayerContainer::new(300,300,300,600, "layer container 1"));
    // container_1.say_hello();
    // let superfaz: Result<LayerContainer, _> = container_1.try_into();
    // // test2.say_hello();
    // assert!(superfaz.is_ok());
    // match superfaz{
    //     Ok(lc)=>{
    //         lc.say_hello()
    //     }
    //     Err(e)=>panic!(e)
    // }
    // NEW_DROPPABLES.lock().unwrap().insert("layer container 1".to_string(), container_1);

use crate::{LayerContainer, DroppableButton, CustomEmmiter, DraggableCb, DragObject, DraggableInfo, LayerDraggable};
use fltk::{app, app::*, group::Pack, group::*, widget::Widget, button::*, draw, enums::*, frame::*, draw::*, prelude::*, widget::*, window::*};
use enum_dispatch::enum_dispatch;
//dynclone for easy cloneing of trait objects
use dyn_clone::DynClone;

#[enum_dispatch(TestDraggable)]
pub trait DraggableExt: DynClone+Send{
    fn is_dragging(&self)->bool;
    fn draggable_id(&self)->String;
    fn set_parent_id(&mut self, new_parent_id: String);
    fn parent_id(&self)->String;
    fn widg(&self)->Box<dyn WidgetExt>;
}

#[enum_dispatch]
#[derive(Clone)]
pub enum TestDraggable{
    DragLayer(LayerDraggable),
}


#[enum_dispatch(TestDroppable)]
pub trait DroppableExt: DynClone+Send{
    fn droppable_id(&self)->String; 
    // fn emmiter(&self)->CustomEmmiter;
    // fn get_child(&mut self, child_id: String)->Option<&mut Box<dyn DraggableExt>>;
    // fn add_child(&mut self, drag_object: DragObject);
    // fn container(&mut self)->Box<dyn GroupExt>; 
    // fn take_child(&mut self, from: &mut Box<dyn DroppableExt + Send>, child_id: String);
    // fn remove_child(&mut self, child_to_remove_id: String)->Option<(String, DragObject)>;
    // fn on_drop(&mut self, drag_source_id: DraggableInfo);
}

#[enum_dispatch]
#[derive(Clone, Debug)]
pub enum TestDroppable{
    LayerContainer(LayerContainer),
    Button(DroppableButton),
    Layer(LayerDraggable)
}

#[enum_dispatch]
pub trait LayerExt: DynClone + Send + DraggableExt{
    fn layer_parent_id(&self)->String;
    fn layer_widg(&self)->Box<dyn WidgetExt>;
    fn set_layer_parent(&mut self, new_parent_id: String);
    fn index(&self)->usize;
    fn set_index(&self, index: usize);
}

#[enum_dispatch(LayerExt, DraggableExt, DroppableExt)]
#[derive(Clone)]
pub enum LayerItem{
    Layer(LayerDraggable)
}


////////////////////////////////
dyn_clone::clone_trait_object!(DraggableExt);
dyn_clone::clone_trait_object!(DroppableExt);





// impl DroppableExt for TestDroppable<D>{
//     match self{
//         LayerContainer(layer_container)=>{
//             layer_con
//         }
//         Button(droppable_button)=>{

//         }
//     }
//     // fn container(&mut self) ->Box<dyn GroupExt>{
//     //     self.container.as_group().unwrap()
//     // }

//     fn droppable_id(&self)->String{
//         match self{
//             LayerContainer(layer_container)=>{
//                 layer_container.droppable_id
//             }
//             Button(droppable_button)=>{
//                 droppable_button.droppable_id
//             }
//         }
//     }
//     fn emmiter(&self)->CustomEmmiter{
//         self.event_manager.clone()
//     }

//     fn get_child(&mut self, id: String)->Option<&mut Box<dyn DraggableExt>>{
//         self.drag_children.get_mut(&id.clone())
//     }

//     fn add_child(&mut self, mut drag_object: DragObject) {
//         dbg!("adding child!");
//         dbg!(self.droppable_id.clone());
//         drag_object.set_parent_id(self.droppable_id.clone());
//         let widg = drag_object.clone().widg();
//         dbg!(drag_object.clone().parent_id());
//         self.drag_children.insert(drag_object.clone().draggable_id(), drag_object);
//         let mut to_add = unsafe{Widget::from_widget_ptr(widg.as_widget_ptr())};
//         self.container.add(&to_add);
//         self.container.redraw();
//         self.container.parent().unwrap().redraw();
//     } 
//     fn take_child(&mut self, from: &mut Box<dyn DroppableExt + Send>, child_id: String){
//         dbg!("Taking Child!");
//         dbg!(child_id.clone());
//     }
//     fn remove_child(&mut self, child_to_remove_id: String) ->Option<(String, DragObject)> {
//         let to_remove_widg = unsafe{Widget::from_widget_ptr(self.drag_children.get(&child_to_remove_id).unwrap().widg().as_widget_ptr())};
//         let index_to_remove = self.container.find(&to_remove_widg);
//         self.container.parent().unwrap().redraw();
//         self.drag_children.remove_entry(&child_to_remove_id.clone())
//     }
//     fn on_drop(&mut self, drag_source: DraggableInfo) {
//     // fn on_drop(&mut self, drag_source: &mut Box<dyn DroppableExt + Send>) {
//         dbg!("got dropped onto layer contiainer");
//     }
// }
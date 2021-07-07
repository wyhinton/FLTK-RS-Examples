use fltk::{app, app::*, group::Pack, image::*, group::*, self, button::*, draw, enums::*, frame::*, draw::*, prelude::*, widget::*, window::*};
use std::sync::{Arc, Mutex};
use crate::{DraggableCb, DND_CHANNEL, traits::*, DraggableType, CustomEmmiter, DragAction, DraggableInfo};
// use state::{Storage, LocalStorage};
use comp_state::{topo, use_state, use_state_current, StateAccess, CloneState};

#[derive(Clone)]
pub struct LayerDraggable{
    frame: Frame,
    draggable_id: &'static str,
    is_dragging: StateAccess<bool>,
    event_emmiter: Arc<Mutex<Option<CustomEmmiter>>>,
    push_cb: Arc<Mutex<Option<DraggableCb>>>,
    parent_id: Arc<Mutex<String>>,
    index: Arc<Mutex<usize>>,
}


impl LayerDraggable{
    #[topo::nested]
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &'static str, chan: Option<CustomEmmiter>)->Self{
        let is_dragging = use_state(||false);
        let push_cb: Option<DraggableCb> = None;
        let mut parent_id = Arc::new(Mutex::new("__ROOT__".to_string()));
        let push_cb = Arc::new(Mutex::new(push_cb));
        let mut frame= Frame::new(x,y,w,h,label);
        let index = Arc::new(Mutex::new(0 as usize));
        
        let event_emmiter: Option<CustomEmmiter> = None;
        let event_emmiter = Arc::new(Mutex::new(event_emmiter));
        frame.set_frame(FrameType::FlatBox);
        frame.set_color(Color::Yellow);
        let parent_id_cl = parent_id.clone();
        let emmiter = event_emmiter.clone();
        frame.handle({
            let push_cb = push_cb.lock().unwrap().clone();
            let index_cl = index.lock().unwrap();
            let drag_info = DraggableInfo::new(parent_id_cl.clone().lock().unwrap().to_string(), label.to_string(), DraggableType::Layer);
            let parent_id_cl = parent_id_cl.clone();
            move |widg, ev| match ev {
                Event::Push => {
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragSource(DraggableInfo::new(parent_id_cl.lock().unwrap().clone(),  label.to_string(), DraggableType::Layer)));
                    true
                }
                Event::Drag =>{
                    dnd();
                    is_dragging.set(true);
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragSource(DraggableInfo::new(parent_id_cl.lock().unwrap().clone(),  label.to_string(), DraggableType::Layer)));
                    widg.set_pos(app::event_coords().0, app::event_coords().1);
                    widg.set_damage(true);
                    true
                 },
                Event::DndEnter=>{
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragTarget(DraggableInfo::new(parent_id_cl.lock().unwrap().clone(),  label.to_string(), DraggableType::Layer)));
                    widg.set_color(Color::Red);
                    widg.redraw();
                    true
                }
                Event::DndLeave=>{
                    widg.set_color(Color::Yellow);
                    widg.redraw();
                    true
                },
                //enables proper reception of other dnd events for the widget
                Event::DndDrag=>{
                    true
                }
                Event::Released=>{
                    // dbg!("got released");
                    true
                }
                Event::DndRelease=>{
                    let under = app::belowmouse::<Frame>().unwrap();
                    if (app::belowmouse::<Frame>().unwrap().is_same(widg)){
                        DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragTarget(DraggableInfo::new(parent_id_cl.lock().unwrap().clone(),  label.to_string(), DraggableType::Layer)));
                        DND_CHANNEL.lock().unwrap().0.send(DragAction::DragEnded);
                        widg.set_color(Color::Yellow);
                        widg.redraw();
                    }
                    true
                }

                _ => false,
            }
        });

        let mut my_widget = LayerDraggable{
            frame: frame,
            draggable_id: label.clone(),
            is_dragging: is_dragging,
            event_emmiter: event_emmiter,
            push_cb: push_cb,
            parent_id: parent_id,
            index: index,
        };
        //enter into global list of draggables
        // DRAGGABLES.get().lock().unwrap().insert(label.to_string(), Box::new(my_widget));
        my_widget
    }
}

impl DraggableExt for LayerDraggable{
    fn is_dragging(&self)->bool{
       CloneState::get(&self.is_dragging)
    }
    fn draggable_id(&self)->String{
        self.draggable_id.to_string()
    }
    fn set_parent_id(&mut self, new_parent_id: String){
        dbg!("setting parent id");
        *self.parent_id.lock().unwrap() = new_parent_id;
    }
    // fn on_drag_end<F: FnMut(&mut Self)+ 'static>(&mut self, cb: F){
    // fn on_drag_end(&self, cb: Box<dyn FnMut(&mut Self)>){
    //     // cb(self)
    //     dbg!(self.draggable_id());
    // }
    // fn with_parent(mut self, p: &mut DragObject)->Self
    // fn with_parent(mut self, p: &mut Box<dyn DroppableExt>)->Self
    //     where Self:Sized{
    //     // *self.event_emmiter.borrow_mut() = Some(p.event_manager());
    //     // let mut to_add = unsafe{Widget::from_widget_ptr(self.frame.as_widget_ptr())};
    //     // *self.parent_id.borrow_mut() = p.droppable_id();
    //     // p.add_child(to_add);
    //     self
    // } 
    fn parent_id(&self)->String{
        let val = self.parent_id.lock().unwrap().to_string().to_string();
        val
    }
    fn widg(&self) ->Box<dyn WidgetExt> {
        Box::new(self.frame.clone())
    }
    // fn set_push_fn(&mut self, cb: DraggableCb) {
    //     *self.push_cb.lock().unwrap() = Some(cb) 
    // }
}

impl LayerExt for LayerDraggable{
    fn layer_widg(&self) ->Box<dyn WidgetExt> {
        Box::new(self.frame.clone())
    }
    fn layer_parent_id(&self)->String{
        let val = self.parent_id.lock().unwrap().to_string().to_string();
        val
    }
    fn set_layer_parent(&mut self, new_parent_id: String){
        dbg!("setting parent id");
        *self.parent_id.lock().unwrap() = new_parent_id;
    }
    fn index(&self) ->usize {
        *self.index.lock().unwrap()
    }
    fn set_index(&self, index: usize){
        *self.index.lock().unwrap() = index;
    }
}

impl DroppableExt for LayerDraggable{
    fn droppable_id(&self)->String{
        self.draggable_id.to_string()
    }
}

impl std::fmt::Debug for LayerDraggable{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layer: {}", self.draggable_id)
    }
}
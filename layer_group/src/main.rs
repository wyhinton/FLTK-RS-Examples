#![warn(clippy::all, rust_2018_idioms)]
use fltk::{app, app::*, group::Pack, image::*, group::*, self, button::*, draw, enums::*, frame::*, draw::*, prelude::*, widget::*, window::*};
use std::ops::{Deref, DerefMut};
use comp_state::{topo, use_state, use_state_current, StateAccess, CloneState};
use uuid::Uuid;
pub mod utils;
use utils::Offset;
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
// use state::LocalStorage;
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
    static ref TEST_DROPPABLE: DropableManager = DropableManager::new();
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

#[derive(Clone)]
pub struct MyFrame{
    frame: Frame,
    draggable_id: &'static str,
    is_dragging: StateAccess<bool>,
    event_emmiter: Arc<Mutex<Option<CustomEmmiter>>>,
    push_cb: Arc<Mutex<Option<DraggableCb>>>,
    parent_id: Arc<Mutex<String>>,
    index: Arc<Mutex<u32>>,
}


impl MyFrame{
    #[topo::nested]
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &'static str, chan: Option<CustomEmmiter>)->Self{
        
        let offset = use_state(|| Offset::new(0, 0));
        let is_dragging = use_state(||false);
        let draggable_id = Uuid::new_v4();
        let push_cb: Option<DraggableCb> = None;
        let mut parent_id = Arc::new(Mutex::new("__ROOT__".to_string()));
        let push_cb = Arc::new(Mutex::new(push_cb));
        let mut frame= Frame::new(x,y,w,h,label);
        let index = Arc::new(Mutex::new(0));
        
        let event_emmiter: Option<CustomEmmiter> = None;
        let event_emmiter = Arc::new(Mutex::new(event_emmiter));
        frame.set_frame(FrameType::FlatBox);
        frame.set_color(Color::Yellow);
        // let my_val = 1; 
        let parent_id_cl = parent_id.clone();
        let emmiter = event_emmiter.clone();
        frame.handle({
            let push_cb = push_cb.lock().unwrap().clone();
        ;
            let index_cl = index.lock().unwrap();
            let drag_info = DraggableInfo::new(parent_id_cl.clone().lock().unwrap().to_string(), *index_cl, label.to_string(), DraggableType::Layer);
            let parent_id_cl = parent_id_cl.clone();
            move |widg, ev| match ev {
                Event::Push => {
                    // dbg!(parent_id_cl.clone());
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragSource(DraggableInfo::new(parent_id_cl.lock().unwrap().clone(),0,  label.to_string(), DraggableType::Layer)));
                    true
                }
                Event::Drag =>{
                    dnd();
                    is_dragging.set(true);
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragSource(DraggableInfo::new(parent_id_cl.lock().unwrap().clone(),0,  label.to_string(), DraggableType::Layer)));
                    offset.set(Offset::from(app::event_coords()));
                    widg.set_pos(app::event_coords().0, app::event_coords().1);
                    widg.set_damage(true);
                    true
                 },
   
                Event::DndEnter=>{
                    // dbg!("got dnd enter");
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragTarget(DraggableInfo::new(parent_id_cl.lock().unwrap().clone(),0,  label.to_string(), DraggableType::Layer)));
                    widg.set_color(Color::Red);
                    widg.redraw();
                    true
                }
                Event::DndLeave=>{
                    widg.set_color(Color::Yellow);
                    widg.redraw();
                    true
                },
                Event::DndLeave=>{
                    true
                }
                //enables proper reception of other dnd events for the widget
                Event::DndDrag=>{
                    true
                }
                Event::Released=>{
                    // dbg!("got released");
                    true
                }
                Event::Paste=>{
                    // dbg!("got paste");
                    true
                }
                Event::DndRelease=>{
                    // dbg!("GOT DND RELEASE");
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragTarget(DraggableInfo::new(parent_id_cl.lock().unwrap().clone(),0,  label.to_string(), DraggableType::Layer)));
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::DragEnded);
                    widg.set_color(Color::Yellow);
                    widg.redraw();
                    true
                }

                _ => false,
            }
        });

        let mut my_widget = MyFrame{
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

impl DraggableExt for MyFrame{
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
    fn on_drag_end(&self, cb: Box<dyn FnMut(&mut Self)>){
        // cb(self)
        dbg!(self.draggable_id());
    }
    // fn with_parent(mut self, p: &mut DragObject)->Self
    fn with_parent(mut self, p: &mut Box<dyn DroppableExt>)->Self
        where Self:Sized{
        // *self.event_emmiter.borrow_mut() = Some(p.event_manager());
        // let mut to_add = unsafe{Widget::from_widget_ptr(self.frame.as_widget_ptr())};
        // *self.parent_id.borrow_mut() = p.droppable_id();
        // p.add_child(to_add);
        self
    } 
    fn parent_id(&self)->String{
        let val = self.parent_id.lock().unwrap().to_string().to_string();
        val
    }
    fn widg(&self) ->Box<dyn WidgetExt> {
        Box::new(self.frame.clone())
    }
    fn set_push_fn(&mut self, cb: DraggableCb) {
        *self.push_cb.lock().unwrap() = Some(cb) 
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

    // let initial_draggables_map = HashMap::new();
    // DRAGGABLES.set(Mutex::new(initial_draggables_map));


    let manager = Rc::new(RefCell::new(DragManager::new()));
    // let manager = Rc::new(RefCell::new(DragManager::new()));
    let app = App::default();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");

    let mut m_cont= Pack::new(200,200,600,600,None);
    m_cont.set_type(PackType::Horizontal);
    m_cont.set_spacing(20);
    let mut container_1= Box::new(LayerContainer::new(300,300,300,600, "layer container 1")) as DropObject;

    let mut my_layer_1 = MyFrame::new(100,100,100,50, "layer 1", None);
    container_1.add_child(Box::new(my_layer_1.clone()) as DragObject);
    dbg!(my_layer_1.parent_id());
    let mut my_layer_2 = MyFrame::new(100,100,100,50, "layer 2", None);
    container_1.add_child(Box::new(my_layer_2) as DragObject);
    let mut my_layer_3 = MyFrame::new(100,100,100,50, "layer 3", None);
    container_1.add_child(Box::new(my_layer_3) as DragObject);
    DROPPABLES.lock().unwrap().insert("layer container 1".to_string(), container_1);
    
    let mut drop_trash = Box::new(DroppableButton::new(50,50,"trash", "assets/delete_black_24dp.svg")) as DropObject;
    DROPPABLES.lock().unwrap().insert(String::from("trash"), drop_trash);
    // let mut trash_can = Frame::new(100,100,50,50, "trash");
    // trash_can.set_frame(FrameType::RFlatBox);
    // trash_can.set_color(Color::Blue);
    // trash_can.draw({
    //     let mut img = SvgImage::load(std::path::Path::new("assets/delete_black_24dp.svg")).unwrap();
    //     move |f| {
    //         img.scale(f.w(), f.h(), true, true);
    //         img.draw(
    //             f.x() + f.w() / 2 - img.width() / 2,
    //             f.y() + f.h() / 2 - img.height() / 2,
    //             img.width(),
    //             img.height(),
    //         );
    //     }
    // });

    let mut container_2= Box::new(LayerContainer::new(700,300,300,600, "layer container 2")) as DropObject;
    let mut my_layer_4 = MyFrame::new(100,100,100,50, "layer 4", None);
    container_2.add_child(Box::new(my_layer_4) as DragObject);
    let mut my_layer_5 = MyFrame::new(100,100,100,50, "layer 5", None);
    container_2.add_child(Box::new(my_layer_5) as DragObject);
    let mut my_layer_6 = MyFrame::new(100,100,100,50, "layer 6", None);
    container_2.add_child(Box::new(my_layer_6) as DragObject);
    m_cont.end();
    DROPPABLES.lock().unwrap().insert("layer container 2".to_string(), container_2);

    let (s,r) = DND_CHANNEL.lock().unwrap().clone();
    let manager_cl = manager.clone();
    let mut display_container= Pack::new(50,100,900,50,"Drag Manager");
    display_container.set_frame(FrameType::FlatBox);
    display_container.set_color(Color::White);

    let source_string = format!("{:?}", manager.borrow().source.clone());
    let mut source_display_frame= Frame::new(0,0,900,20, None);
    source_display_frame.set_frame(FrameType::BorderFrame);
    source_display_frame.set_color(Color::Blue);
    source_display_frame.set_label(&source_string);
    let mut frame_cl = source_display_frame.clone();
    
    // let under_string = format!("{:?}", manager.borrow().under.clone());
    // let mut under_display_frame= Frame::new(0,0,900,20, None);
    // under_display_frame.set_frame(FrameType::BorderFrame);
    // under_display_frame.set_color(Color::Blue);
    // under_display_frame.set_label(&under_string);
    
    let release_string = format!("{:?}", manager.borrow().under.clone());
    let mut target_display_frame= Frame::new(0,0,900,20, None);
    target_display_frame.set_frame(FrameType::BorderFrame);
    target_display_frame.set_color(Color::Blue);
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
    app::add_idle(move || {
        //using recv_timeout() is essential for saving CPU
        if let Ok(msg) = r.recv_timeout(Duration::from_millis(20)) {
            let source_string = format!("Source: {:?}", manager.borrow_mut().source.clone());
            source_display_frame.set_label(&source_string);
            match msg {
                DragAction::SetDragTarget(info) => {
                    dbg!(format!("Setting Drag Target To: {}", info.draggable_id.clone()));
                    // dbg!(info.clone());
                    manager_cl.borrow_mut().target = Some(info.clone());
                    target_display_frame.redraw();
                    target_display_frame.set_damage(true);
                    target_display_frame.set_label(&format!("target: {:?}", info.clone()));
                }, 
                DragAction::DragMove(info)=>{
                    dbg!("got enter ");
                }
                DragAction::DragLeave=>{
                    dbg!("got enter ");
                    // dbg!(info.clone());
                    // under_display_frame.set_label("under: None ");
                    // manager_cl.borrow_mut().under = None;
                }
                DragAction::SetDragSource(info)=>{
                    dbg!("setting source");
                    manager_cl.borrow_mut().source = Some(info.clone());
                    source_display_frame.set_label(&format!("source: {:?}", info.clone()));
                    source_display_frame.redraw();
                    source_display_frame.set_damage(true);
                    source_display_frame.parent().unwrap().redraw();

                }
                DragAction::DragEnded=>{
                    dbg!("drag eneded!");
                    dbg!(manager_cl.borrow());

                    match (manager_cl.borrow().source.clone(), manager_cl.borrow().target.clone()){
                        (Some(source), Some(target))=>{
                            // let pair = get_mut_pair::<String, DropObj>::(DROPPABLES.lock().unwrap(), source.draggable_id, target.draggable_id);
                            // let pair = get_mut_pair(&mut DROPPABLES.lock().unwrap(), &source.parent.clone(), &target.parent.clone());
                            
                            // DROPPABLES.lock().unwrap().get_mut(&source.parent.clone()).on_drop(source);
                             match (source.draggable_type.clone(), target.draggable_type.clone()){
                                 //(Source, Target)
                                (DraggableType::Layer, DraggableType::Layer)=>{
                                    dbg!("dragged from layer to layer");
                                    let to_add_to_target = DROPPABLES.lock().unwrap().get_mut(&source.parent.clone()).unwrap().remove_child(source.draggable_id).expect("failed to get source child");
                                    DROPPABLES.lock().unwrap().get_mut(&target.parent.clone()).unwrap().add_child(to_add_to_target.1);
                                },
                                (DraggableType::Layer, DraggableType::Container)=>{
                                    dbg!("dragged from a layer to a container");
                                }
                                (DraggableType::Layer, DraggableType::Button)=>{
                                    dbg!("TARGET WAS BUTTON");
                                    // let mut all =  DROPPABLES.lock().unwrap();
                                    // dbg!("HELLO");
                                    // dbg!(DROPPABLES.lock().unwrap().get(&target.draggable_id.clone()).unwrap().droppable_id());
                                    // let mut pair = get_mut_pair(&mut DROPPABLES.lock().unwrap(), &source.parent.clone(), &target.draggable_id.clone());
                                    // dbg!(&pair.0.droppable_id());
                                    // let test = pair.1.on_drop(pair.0);
                                    DROPPABLES.lock().unwrap().get_mut(&target.draggable_id).unwrap().on_drop(source.clone());
                                    dbg!("dragged from layer to button");
                                }
                                _=>({dbg!("it was something else");})
                             }
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

fn get_mut_pair<'a, K, V>(conns: &'a mut HashMap<K, V>, a: &K, b: &K) -> (&'a mut V, &'a mut V)
where
    K: Eq + std::hash::Hash,
{
    unsafe {
        let a = conns.get_mut(a).unwrap() as *mut _;
        let b = conns.get_mut(b).unwrap() as *mut _;
        assert_ne!(a, b, "The two keys must not resolve to the same value");
        (&mut *a, &mut *b)
    }
}
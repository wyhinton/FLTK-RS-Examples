use std::{sync::{Arc, Mutex}};
use fltk::{prelude::*, enums::*, group};

#[derive(Clone)]
pub struct ArcWidget<T: WidgetExt + WidgetBase >(Arc<Mutex<T>>);

impl<T: WidgetExt + WidgetBase> ArcWidget<T>{    
    pub fn new(widg: T)->Self{
        ArcWidget(Arc::new(Mutex::new(widg)))
    }
    pub fn x(&self)->i32{
        let widg = self.0.lock().unwrap();
        widg.x()
    }
    pub fn y(&self)->i32{
        let widg = self.0.lock().unwrap();
        widg.y()
    }
    pub fn width(&self)->i32{
        let widg = self.0.lock().unwrap();
        widg.width()
    }
    
    pub fn height(&self)->i32{
        let widg = self.0.lock().unwrap();
        widg.height()
    }
    pub fn set_size(&self, width: i32, height: i32){
        let mut widg = self.0.lock().unwrap();
        widg.set_pos(width, height);
    }
    pub fn redraw(&self){
        let mut widg = self.0.lock().unwrap();
        widg.redraw();
    }
    pub fn label(&self)->String{
        let widg = self.0.lock().unwrap();
        widg.label()
    }
    pub fn draw<F: FnMut(&mut T) + 'static>(&self, cb: F){
        let mut widg = self.0.lock().unwrap();
        widg.draw(cb)
    }
    pub fn handle<F: FnMut(&mut T, Event)->bool +'static>(&self, cb: F){
        let mut widg = self.0.lock().unwrap();
        widg.handle(cb)
    }
    pub fn parent(&self)->Option<group::Group>{
        let widg = self.0.lock().unwrap();
        widg.parent() 
    }
    pub fn resize(&self, x: i32, y: i32, width: i32, height: i32){
        let mut widg = self.0.lock().unwrap();
        widg.resize(x,y,width,height); 
    }
    pub fn set_width(&self, w: i32){
        let x = self.0.lock().unwrap().x();
        let y = self.0.lock().unwrap().y();
        let height = self.0.lock().unwrap().height();
        let mut widg = self.0.lock().unwrap();
        widg.resize(x,y,w,height); 
    }
    pub fn set_height(&self, h: i32){
        let x = self.0.lock().unwrap().x();
        let y = self.0.lock().unwrap().y();
        let width = self.0.lock().unwrap().width();
        let mut widg = self.0.lock().unwrap();
        widg.resize(x,y,width,h); 
    }
    pub fn set_y(&self, y: i32){
        let x = self.0.lock().unwrap().x();
        let width = self.0.lock().unwrap().width();
        let height = self.0.lock().unwrap().height();
        let mut widg = self.0.lock().unwrap();
        widg.resize(x,y,width,height); 
    }
    pub fn set_x(&self, x: i32){
        let y = self.0.lock().unwrap().y();
        let mut widg = self.0.lock().unwrap();
        widg.set_pos(x,y); 
    }
    pub fn set_frame(&self, frame: FrameType){
        let mut widg = self.0.lock().unwrap();
        widg.set_frame(frame); 
    }
    pub fn set_color(&self, color: Color){
        let mut widg = self.0.lock().unwrap();
        widg.set_color(color); 
    }
    pub fn set_pos(&self, x: i32, y: i32){
        let mut widg = self.0.lock().unwrap();
        widg.set_pos(x,y);
    }
    pub fn set_damage_type(&self, d: Damage){
        let mut widg = self.0.lock().unwrap();
        widg.set_damage_type(d);
    }

}



#[derive(Clone)]
pub struct ArcGroup<T: WidgetExt + WidgetBase + GroupExt + Clone>(Arc<Mutex<T>>);

impl<T: WidgetExt + WidgetBase + GroupExt + Clone> ArcGroup<T>{    
    pub fn widg(&self)->T{
        self.0.lock().unwrap().clone()
    }

    pub fn new(widg: T)->Self{
        ArcGroup(Arc::new(Mutex::new(widg)))
    }
    pub fn x(&self)->i32{
        let widg = self.0.lock().unwrap();
        widg.x()
    }
    pub fn y(&self)->i32{
        let widg = self.0.lock().unwrap();
        widg.y()
    }
    pub fn width(&self)->i32{
        let widg = self.0.lock().unwrap();
        widg.width()
    }
    pub fn height(&self)->i32{
        let widg = self.0.lock().unwrap();
        widg.height()
    }
    pub fn set_size(&self, width: i32, height: i32){
        let mut widg = self.0.lock().unwrap();
        widg.set_pos(width, height);
    }
    pub fn redraw(&self){
        let mut widg = self.0.lock().unwrap();
        widg.redraw();
    }
    pub fn label(&self)->String{
        let widg = self.0.lock().unwrap();
        widg.label()
    }
    pub fn draw<F: FnMut(&mut T) + 'static>(&self, cb: F){
        let mut widg = self.0.lock().unwrap();
        widg.draw(cb)
    }
    pub fn end(&self){
        let widg = self.0.lock().unwrap();
        widg.end();
    }
    pub fn handle<F: FnMut(&mut T, Event)->bool +'static>(&self, cb: F){
        let mut widg = self.0.lock().unwrap();
        widg.handle(cb)
    }
    pub fn children(&self)->i32{
        let widg = self.0.lock().unwrap();
        widg.children()
    }
    pub fn parent(&self)->Option<group::Group>{
        let widg = self.0.lock().unwrap();
        widg.parent() 
    }
    pub fn resize(&self, x: i32, y: i32, width: i32, height: i32){
        let mut widg = self.0.lock().unwrap();
        widg.resize(x,y,width,height); 
    }
    pub fn set_frame(&self, frame: FrameType){
        let mut widg = self.0.lock().unwrap();
        widg.set_frame(frame); 
    }
    pub fn set_color(&self, color: Color){
        let mut widg = self.0.lock().unwrap();
        widg.set_color(color); 
    }
    pub fn set_width(&self, w: i32){
        let x = self.0.lock().unwrap().x();
        let y = self.0.lock().unwrap().y();
        let height = self.0.lock().unwrap().height();
        let mut widg = self.0.lock().unwrap();
        widg.resize(x,y,w,height); 
    }
    pub fn set_height(&self, h: i32){
        let x = self.0.lock().unwrap().x();
        let y = self.0.lock().unwrap().y();
        let width = self.0.lock().unwrap().y();
        let mut widg = self.0.lock().unwrap();
        widg.resize(x,y,width,h); 
    }
    pub fn set_y(&self, y: i32){
        let x = self.0.lock().unwrap().x();
        let width = self.0.lock().unwrap().width();
        let height = self.0.lock().unwrap().height();
        let mut widg = self.0.lock().unwrap();
        widg.resize(x,y,width,height); 
    }
    pub fn set_x(&self, x: i32){
        let y = self.0.lock().unwrap().y();
        let mut widg = self.0.lock().unwrap();
        widg.set_pos(x,y); 
    }
}
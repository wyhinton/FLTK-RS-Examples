use serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub enum DraggableType{
    Layer,
    Container,
    Button,
}

pub enum DroppablType{
    Layer,
    Container, 
    Button,
}


#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct DraggableInfo{
    pub parent: String,
    // pub index: u32,
    pub draggable_id: String,
    pub draggable_type: DraggableType,
}

#[derive(Clone, Debug)]
pub enum DragAction{
    DragPickUp(DraggableInfo),
    DragRelease(DraggableInfo),
    DragMove(DraggableInfo),
    DraggingOver(DraggableInfo),
    SetDragSource(DraggableInfo),
    SetDragTarget(DraggableInfo),
    DragEnded,
    DragLeave,
}



// pub fn on<F, T>(&self, event: &str, callback: F) -> String
// where 
//     for<'de> T: Deserialize<'de>,
//     F: Fn(T) + 'static + Sync + Send 
// {
//     self.0.lock().unwrap().on(event, callback)
//     // let id = self.on_limited(event, None, callback);
//     // return id;
// }

impl DraggableInfo{
    pub fn new(parent: String, draggable_id: String, draggable_type: DraggableType)->Self{
        DraggableInfo{
            parent: parent.clone(),
            draggable_id: draggable_id.clone(),
            draggable_type: draggable_type,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub enum WidgetValue{
    Integer32(i32),
    Unsized32(u32),
    CString(String),
}


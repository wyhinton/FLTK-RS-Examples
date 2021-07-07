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

// #[serde(untagged)]
// #[derive(Clone, Serialize, Deserialize, Debug, Hash)]
// // #[serde(bound(deserialize = "'de: 'static"))]
// pub struct DraggableInfo{
//     pub parent: WidgetValue,
//     pub index: WidgetValue,
//     pub draggable_id: WidgetValue,
//     pub draggable_type: DraggableType,
// }
#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
// #[serde(bound(deserialize = "'de: 'static"))]
pub struct DraggableInfo{
    pub parent: String,
    pub index: u32,
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
    pub fn new(parent: String, index: u32, draggable_id: String, draggable_type: DraggableType)->Self{
        DraggableInfo{
            parent: parent.clone(),
            index: index,
            draggable_id: draggable_id.clone(),
            draggable_type: draggable_type,
            
        }
        // DraggableInfo{
        //     parent: WidgetValue::CString(parent.to_string()),
        //     index: WidgetValue::Unsized32(index),
        //     draggable_id: WidgetValue::CString(draggable_id.to_string()),
        //     draggable_type: draggable_type,
            
        // }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub enum WidgetValue{
    Integer32(i32),
    Unsized32(u32),
    CString(String),
}



// #[derive(Clone, Copy, Serialize, Deserialize, Debug)]
// pub enum MyEnum{
//     Integer32(i32),
//     Unsized32(u32),
//     SomeString(String),
// }


// fn decode_enum_to_string(e: MyEnum)->Result<String, Error>{
//     //returns a result with our decoded string value, or an error 
//     //if the decoding fails
// }
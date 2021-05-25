use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*};
pub mod card;
use card::{Card};
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::Rc;
use lazy_static::lazy_static;
use history_tree::HistoryTree;

#[derive(Clone)]
pub struct AppState{
    gallery: Gallery,
    sender: app::Sender<Message>,
    ht: HistoryTree,
}

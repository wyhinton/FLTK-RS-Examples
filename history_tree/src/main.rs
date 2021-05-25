use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*};
pub mod card;
use card::{Card};
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::Rc;
use lazy_static::lazy_static;
use history_tree::HistoryTree;

static GALLERY_HEIGHT: i32 = 200;
static IMAGE_HEIGHT: u32 = GALLERY_HEIGHT as u32 - 20;
static WINDOW_WIDTH: i32 = 1000;
static GALLERY_WIDTH: i32 = 800;
static WINDOW_HEIGHT: i32 = 400;

lazy_static::lazy_static! {
    pub static ref CHANNEL: (app::Sender<Message>, app::Receiver<Message>) = app::channel();
}
#[derive(Debug, Clone)]
pub struct Gallery{
    pack: Pack,
    cards: Vec<Card>,
    // sender: app::Sender<Message>,
}


impl Gallery{
    fn new(cards: Vec<Card>, x: i32, y: i32, w: i32, h:i32)->Gallery{
        let mut gallery_container = Pack::new(x,y,w,h, "");
        gallery_container.end();
        gallery_container.set_type(PackType::Horizontal);
        
        for x in 0..cards.len(){
            gallery_container.add(&cards[x].pack);
        }
        let g = Gallery{
            pack: gallery_container,
            cards: cards.clone(), 
        };
        g
    }
    fn add_card(&mut self){
        let nc = Card::new("imgs/chrome.jpg").unwrap();
        self.cards.push(nc.clone());
        self.pack.add(&nc.pack);
    }
    fn change_card(&mut self, card_id: Uuid){
        for x in 0..self.cards.len(){
            if self.cards[x].id == card_id{
                self.cards[x].set_image("imgs/forest.jpg").unwrap()
            }
        }
    }
}
#[derive(Clone)]
pub struct AppState{
    gallery: Gallery,
    sender: app::Sender<Message>,
    ht: HistoryTree,
}

impl AppState{
    pub fn root(&self) -> usize {self.ht.root()}
    pub fn add(&mut self, card: Card, parent: usize) -> usize {
        let cursor = self.ht.cursor();
        self.gallery.add_card();
        self.ht.add(parent)
    }
    pub fn change(&mut self, img_path: &str, node: &mut usize) {
        let cursor = self.ht.cursor();
        self.gallery.
        self.ht.change(node);
    }
}

type AppRef = Rc<RefCell<AppState>>;
#[derive(Clone)]
struct AppContainer(AppRef);

impl AppContainer{
    fn new(inner: AppState) -> AppContainer{
        // let s: app::Sender<Message> =  CHANNEL.0.clone();3
        let app = AppState{
            gallery: inner.gallery.clone(),
            sender: inner.sender.clone(),
            ht: HistoryTree::new(),
            // history: inner.history.clone(),
        };
        let app_cl = app.clone();
        return AppContainer(Rc::new(RefCell::new(app)));
    }
    fn delete_card(&self, id: Uuid) {
        (self.0.borrow_mut()).gallery.cards.retain(|x| x.id != id)
    }
    fn add_card(&self, path: String){
        let s = (self.0.borrow_mut()).sender.clone();
        let new_card = Card::new(&path).unwrap();
        (self.0.borrow_mut()).gallery.cards.push(new_card);
    }
}

#[derive(Debug, Clone)]
pub enum Message{
    DeleteCard(Uuid),
    AddCard(String),
}


fn main() {
    let app = App::default();
    let mut win = Window::new(200, 200, WINDOW_WIDTH, WINDOW_HEIGHT, "");
    
    let mut controls = Pack::new(100,80,150,20,"");
    let mut add_button = Button::new(0,0,100,20,"New Random Image");
    add_button.set_color(Color::Green);
    add_button.emit(CHANNEL.0.clone(), Message::AddCard("imgs/chrome.jpg".to_string()));
    controls.end();

    let def_cards = vec![
        Card::new("imgs/chrome.jpg").unwrap(),
        Card::new("imgs/forest.jpg").unwrap(),
        Card::new("imgs/smiley.png").unwrap(),
    ];


    let gallery = Gallery::new(def_cards, (WINDOW_WIDTH-GALLERY_WIDTH)/2,100,800,300);
    win.end();
    win.show();

    let initial_app_state = AppState{
        gallery: gallery,
        // sender: C
        sender: CHANNEL.0.clone(),
        ht: HistoryTree::new(),
    };
    let container = AppContainer::new(initial_app_state.clone());

    while app.wait() {
        if let Some(msg) = CHANNEL.1.recv() {
            use Message::*;
            match msg{
                DeleteCard(id) => {
                    container.delete_card(id);
                    dbg!("should delete card");
                }
                AddCard(path)=>{
                    container.add_card("imgs/chrome.jpg".to_string());
                    dbg!("should add card");
                }
            }
        }
    }
}


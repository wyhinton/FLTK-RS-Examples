//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*};
pub mod card;
use card::{Card};
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::Rc;
use undo::{Action, History};
use lazy_static::lazy_static;

static GALLERY_HEIGHT: i32 = 200;
static IMAGE_HEIGHT: u32 = GALLERY_HEIGHT as u32 - 20;
static WINDOW_WIDTH: i32 = 1000;
static GALLERY_WIDTH: i32 = 800;
static WINDOW_HEIGHT: i32 = 400;
// https://gist.github.com/DanielKeep/18022271ef399c063177
#[derive(Debug, Clone)]
pub struct Gallery{
    pack: Pack,
    cards: Vec<Card>,
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
}
#[derive(Clone)]
pub struct AppState{
    gallery: Gallery,
    sender: fltk::app::Sender<Message>,
    // history: History<Action>, 
}

struct AppData{
    cards: Vec<Card>,
}

struct AddCard(Card);
impl Action for AddCard{
    type Target = Card;
    type Output = ();
    type Error = &'static str;
    
    fn apply(&mut self, s: &mut Card) -> undo::Result<AddCard> {
        // s.push(self.0);
        Ok(())
    }

    fn undo(&mut self, s: Card) -> undo::Result<AddCard> {
        // self.0 = s.pop().ok_or("s is empty")?;
        Ok(s)
    }
}

// other.on_set_call(move |&mut: v| o_cl.borrow().notify(v));

impl std::fmt::Debug for AppState{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result{
        fmt.debug_struct("AppState")
        .field("gallery", &self.gallery)
        .finish()
   }
}


type AppRef = Rc<RefCell<AppState>>;
#[derive(Clone)]
struct AppContainer(AppRef);

impl AppContainer{
    fn new(inner: AppState) -> AppContainer{
        let app = AppState{
            gallery: inner.gallery.clone(),
            sender: CHANNEL.0.clone(),
            history: inner.history.clone(),
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
    pub fn notify(self, value: i32) {
        println!("Obj1 notified: {}", value);
    }
}

#[derive(Debug, Clone)]
pub enum Message{
    DeleteCard(Uuid),
    AddCard(String),
}

lazy_static::lazy_static! {
    pub static ref CHANNEL: (app::Sender<Message>, app::Receiver<Message>) = app::channel();
}

fn main() {
    let app = App::default();
    // let (s, r) = channel::<Message>();

    // GLOBAL.set(initial_app_state);
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
        sender: CHANNEL.0.clone(),
        // history: History::new(),
    };
    let container = AppContainer::new(initial_app_state.clone());

    while app.wait() {
        if let Some(msg) = CHANNEL.1.recv() {
            use Message::*;
            match msg{
                DeleteCard(id) => {
                    container.delete_card(id);
                    // dbg!("should delete card");
                }
                AddCard(path)=>{
                    
                    container.add_card("imgs/chrome.jpg".to_string());
                    // dbg!("should add card");
                }
            }
        }
    }
}


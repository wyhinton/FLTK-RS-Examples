//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*};
use sublime_fuzzy::best_match;
use rand::{distributions::Alphanumeric, Rng};

mod search_bar;
use search_bar::SearchBar;
use search_bar::Result;

mod search_table;
use search_table::SearchTable;

// #[derive(Debug, Clone, Send, Sync)]
#[derive(Debug, Clone)]
pub enum Message {
    Test,
    UpdateSearch(Vec<Result>),
    RedrawTable,
    ResetTable,
    SetNumStrings(i32),
    
}
    ///generate array of random ASCII strings
fn random_string_arr(n_strings: i32, min_str_length: i32, max_str_length: i32) -> Vec<String>{
    let mut string_arr = vec![];
    
    for x in 0..n_strings{
        let mut count =  rand::thread_rng().gen_range(min_str_length..max_str_length);
        println!("rand char count is:{}", count.to_string());
        let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

        string_arr.push(s)
    }
    string_arr
 }

fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Fuzzy Search");
    let initial_amount = 100; 
    let initial_strings = random_string_arr(initial_amount, 5, 30);

    let sb_width = 600;
    let mut sb = SearchBar::new((win.width()/2)-sb_width/2,200,sb_width,25, initial_strings.clone(), s.clone());
    let mut table = SearchTable::new(sb.x(),sb.y()+sb.height(), sb_width, 500, initial_strings.clone(), vec![], s.clone()) ;

    let ip_width = 100;
    let input_pack = Pack::new(win.width()/2 - ip_width/2, 100, ip_width, 50, "");
        let mut num_strings_input = Input::new(0,0, 50, 25, "Number of Random Strings");
        num_strings_input.set_type(InputType::Int);
        num_strings_input.set_value(&initial_amount.to_string());
    input_pack.end();

    let nsis = s.clone();
    num_strings_input.set_callback2(move |widg| {
        println!("got input cb {}", "");
        match widg.value().parse(){
            Ok(n)=>nsis.send(Message::SetNumStrings(n)),
            Err(n) => println!("{}", "not a valid input"),
        }
    });

    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    println!("{}", "got test message");
                    app::redraw();
                }
                UpdateSearch(results) =>{
                    println!("results at main are {:?}", results);
                    println!("{}", "got update search message");
                    table.update(results.clone(), s.clone());
                }
                RedrawTable => {
                    table.redraw();
                }
                ResetTable => {
                    table.reset(s.clone());
                }
                SetNumStrings(num) => {
                    let nr = random_string_arr(num, 5, 20);
                    table.set_items(nr.clone());
                    sb.set_items(nr.clone(), s.clone());
                    table.reset(s.clone());
                    
                }
            }
        }
    }
}


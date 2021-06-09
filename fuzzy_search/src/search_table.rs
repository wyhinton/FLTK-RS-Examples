use fltk::{prelude::*,  table::*, };
use super::Result;
use super::Message;

mod table_button;
use table_button::TableButton;

use std::ops::{Deref, DerefMut};




pub struct SearchTable{
    table: Table,
    values: Vec<Result>,
    items: Vec<String>
}
fn closest_multiple(mut n: i32, mut x: i32) -> i32 { 
    let result = ((n + x/2) / x) * x;
    return result; 
} 

fn format_table(num_cols: i32, p_width: i32, t: &Table, item_count: usize){
    let mut tc = t.clone();
    println!("closet multiple is {}", closest_multiple(num_cols as i32, item_count as i32));
    tc.set_rows(20);
    tc.set_cols(num_cols);
    tc.set_row_height_all(90);
    tc.set_col_width_all((p_width/num_cols as i32));
}

impl SearchTable{
    pub fn new(x: i32, y: i32, w: i32, h: i32, all_items: Vec<String>, s_items: Vec<Result>, s: fltk::app::Sender<Message>) -> SearchTable {
        let mut st = SearchTable{
            table: Table::new(x, y, w+20, h, ""),
            values: s_items.clone(), 
            items: all_items.clone(),
        };
        let ic = st.items.clone(); 
        
        format_table(8, w, &st.table, ic.len());        
        st.table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
            TableContext::Cell => {
                let t_index = row*t.cols() as i32+ col;
                let mut button = TableButton::new(x,y,w,h);
                button.set_label(&ic[t_index as usize]);
                t.add(&button.wid);
            },
            _ => ()
        });
        
        //we need to send a message to redraw the app after the table has been drawn
        s.send(Message::RedrawTable);
        st.table.end();

        st
    }
    //draw only the buttons with a match score greater than 0
    pub fn update(&mut self, search_results: Vec<Result>, s: fltk::app::Sender<Message>){
        fltk::prelude::GroupExt::clear(&mut self.table);
        format_table(8, self.table.width(), &self.table, search_results.len());       

        self.table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
            TableContext::Cell => {
                let t_index = row*t.cols() as i32+ col;
                if t_index < search_results.len() as i32{
                    let cur_res = &search_results.clone()[t_index as usize];
                    if cur_res.score > 0 {
                        let mut button = TableButton::new(x,y,w,h);
                        button.set_label(&cur_res.val.to_string());
                        &button.redraw();
                        t.add(&button.wid);
                        
                    }
                }
            },
            _ => ()
        });
        s.send(Message::RedrawTable);
        
    
    }
    //reset table to show all values
    pub fn reset(&mut self, s: fltk::app::Sender<Message>){
        println!("{}", "should reset table" );
        fltk::prelude::GroupExt::clear(&mut self.table);
        format_table(8, self.table.width(), &self.table, self.items.len());   
        let ric = self.items.clone();    
        self.table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
            TableContext::Cell => {
                let t_index = row*t.cols() as i32 + col;
                println!("all items len at reset is {}", ric.len().to_string());
                if t_index < ric.len() as i32 {
                    let mut button = TableButton::new(x,y,w,h);
                    button.set_label(&ric[t_index as usize]);
                    button.set_callback(|b| println!("Selected: {}", b.label()));
                    t.add(&button.wid);  
                }
            },
            _ => ()
        });
        s.send(Message::RedrawTable);
    }

    pub fn set_items(&mut self, new_items: Vec<String>){
        self.items = new_items.clone();
        println!("new set items are {:?}", new_items);
    }

}

impl Deref for SearchTable {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl DerefMut for SearchTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

use fltk::{prelude::*,  table::*, button::*,group::*, image::*, enums::*};
use crate::{CustomEvent};
use std::ops::{Deref, DerefMut};
pub struct SearchTable{
    table: Table,
    items: Vec<String>,
    buttons: Vec<Button>,
}

impl SearchTable{
    pub fn new(x: i32, y: i32, w: i32, h: i32, items: Vec<String>) -> SearchTable {
        //table setup
        let mut table = Table::new(x,y,w+20,h, None);
        table.end();
        table.set_rows(10);
        table.set_cols(5);
        table.set_row_height_all(90);
        
        //draw our widgets again if we input into our search bar
        table.handle(move |widg, ev| 
          if ev == CustomEvent::SEARCH_INPUT.into(){
              widg.redraw();
            true
        } else {
            false
        });
        

        let mut buttons = vec![];
        let image = SvgImage::load("search-24px.svg").unwrap();
        for _ in 0..100{
            let mut button = Button::new(0,0,50,50, None);
            button.set_frame(FrameType::FlatBox);
            // button.set_color(Color::Red);
            button.set_image(Some(image.clone()));
            buttons.push(button.clone());
            table.add(&button)
        }

        layout_table(&mut table, items.clone(), buttons.clone());

        let st = SearchTable{
            table: table,
            items: items.clone(),
            buttons: buttons,
        };
        st
    }
    //update our available list of strings
    pub fn set_values(&mut self, new_items: Vec<String>){
        self.items = new_items.clone();
        let button_cl = self.buttons.clone();
        dbg!(new_items.clone().len());
        let mut queue_group = Group::new(600,0,100,100,None);
        // queue_gro
        //remove buttons that dont need to be drawn
        for x in 0..button_cl.len()-new_items.len(){
            queue_group.add(&button_cl[x as usize]);
            // dbg!(self.table.children());
        }
        //for every one of our strings add a button
        for x in 0..new_items.len(){
            self.table.add(&button_cl[x as usize]);
        }
        //setting a minimum number of rows prevents funky redrawing issues
        //draw our table
        layout_table(&mut self.table, new_items.clone(), button_cl);
    }

}
fn layout_table(table: &mut Table, strings: Vec<String>, mut buttons: Vec<Button>){
    let num_rows = (strings.len() as f32/table.cols() as f32).ceil();
    table.set_rows(std::cmp::max(num_rows as i32, 1));

    table.draw_cell(move |t, ctx, row, col, x, y, w, h| {
        if let TableContext::Cell = ctx {
            let t_index = row*t.cols() as i32+ col;
            //reposition, redraw, and relabel the buttonsw with the provided strings
            if t_index < *&strings.len() as i32{
                buttons[t_index as usize].resize(x,y,w,h);
                buttons[t_index as usize].redraw();
                buttons[t_index as usize].set_label(&strings[t_index as usize] );
                t.redraw();
                t.add(&buttons[t_index as usize]);
            }
        }
    });
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

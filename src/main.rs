pub mod enums;
pub mod types;
pub mod data;
pub mod file;

use std::{io::{self}, vec, fs, path::Path, collections::HashMap};
use cursive::{views::{LinearLayout, Canvas, TextView, Panel}, theme::{Theme, Palette, ColorStyle}, view::Resizable, Printer, CursiveRunnable, XY, Cursive};
use cursive::theme::BaseColor::{Black, White};
use cursive::traits::Nameable;
use cursive::theme::Color;
use cursive::theme::PaletteColor::{Background, Primary, View};
use data::create_stuff;
use rand::prelude::*;
use types::{Item, GameState};
use crate::types::Size;
use crate::file::load_items;
fn main() {
    
    let mut item_map:HashMap<u16, Box<dyn Item>> = HashMap::new();
    load_items(&mut item_map);
    for item in item_map.into_iter() {
        let item_box = item.1;
        let item_name = item_box.get_name;
        println!("x");
    }
    //let item_name = item_1.get_name();
    let state: GameState = GameState::new();
    //let graphics: Vec<Box<String>> = load_graphics();
    //let mut siv = cursive::default();
    //setup_window(&mut siv, &graphics);
    //write_to("output", &String::from("Welcome to The Hallway!"), &mut siv);
    let mut rng = rand::thread_rng();
    let items = create_stuff();
    
    // for i in 0..32 {
    //     println!("{0}", gen_name(&mut rng));
    // }
    let mut inventory: Vec<&Box<dyn Item>> = vec![];
    let mut state = String::from("standing in");
    let mut item_roll: Option<&Box<dyn Item>> = None;
    // loop {
    //     println!("You are {0} a hallway...", state);
    //     print!("Enter a command <z = get, c = continue, x = wait, q = quit>: ");
    //     io::stdout().flush().unwrap();
    //     let res = process_input(&mut state, item_roll, &mut inventory);
    //     if res == 1 {
    //         break;
    //     }
    //     item_roll = gen_item(&mut rng, &items);
    // }

    //siv.run();
}

pub fn load_graphics() -> Vec<Box<String>> {
    let g_path_str = String::from("./target/debug/gate.txt");
    let g_path = Path::new(&g_path_str);
    let gate = fs::read_to_string(g_path);
    if gate.is_ok() { 
        vec![Box::new(gate.unwrap())]
    } else {
        vec![]
    }

}

pub fn setup_window(siv: &mut CursiveRunnable, graphics: &Vec<Box<String>>) {
    siv.add_global_callback('q', |s| s.quit());
    //siv.on_event()
    //let gate = graphics[0].as_ref();
    let mut palette = Palette::default();
    palette.extend(vec![(Background, Color::Dark(Black)), (View, Color::Dark(Black)), (Primary, Color::Light(White))]);
    
    let theme = Theme {
        shadow: false,
        borders: cursive::theme::BorderStyle::Simple,
        palette
    };

    siv.set_theme(theme);

    let info_panel = Panel::new(
        TextView::new(String::new())
        .with_name("info")
        .fixed_size(XY::new(20, 30)));
    
    let room_panel = Panel::new(
        Canvas::new(String::new()) 
        .with_name("room")
        .fixed_size(XY::new(96, 26))
    );
    
    let output_panel = Panel::new(
        TextView::new(String::new())
        .with_name("output")
        .fixed_size((96, 4))
    );
     
    // let status_bar = TextView::new(String::from(""));

    siv.add_layer(LinearLayout::horizontal().child(info_panel)
        .child(LinearLayout::vertical()
            .child(room_panel)
            .child(output_panel))
    );
}

pub fn new_game() {

}

pub fn gen_item<'a>(rng: &mut ThreadRng, items: &'a Vec<Box<dyn Item>>) -> Option<&'a Box<dyn Item>> {
    let chance:usize = rng.gen_range(1..10);
    if chance <= items.len() {
        let index = chance-1;
        let item = &items[index];
        println!("There is a {0} here!", item.as_ref().get_name());
        Some(item)
    } else {
        None        
    }
}

pub fn draw_room(room_str: &String, p: &Printer<'_, '_>) {
    let style = ColorStyle::new(
        Color::Rgb(255, 255, 255),
        Color::Rgb(0, 0, 0)
    );
    p.print_box((0, 0), p.size, false);
    let sz: Size = Size::new(p.size.x, p.size.y);
    for x in 1..sz.x-1 {
        for y in 1..sz.y-1 {
            p.with_color(style, |printer| {
                printer.print((x, y), ".");
            })
        }
    }
}

pub fn gen_name(rng: &mut ThreadRng) -> String {
    let c_digraphs = vec!["sh", "kn", "ch", "ph", "wr", "ck", "ss", "tch", "th", "wh", "qu", "tr", "cr", "ly", "ry", "cy", "by"];
    let v_digraphs = vec!["ai", "ay", "ee", "ea", "ie", "ei", "oo", "ou", "ow", "oe", "ue", "ey", "ay", "oy", "oi", "au", "aw"];
    let consonants = vec!["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s", "t", "v", "w", "x", "y", "z"];
    let vowels = vec!["a", "e", "i", "o", "u"];
    let length = rng.gen_range(3..10);
    let mut name = String::new();
    let mut cat = 0;
    let mut cons: bool = rng.gen_bool(0.5);
    let mut list: &Vec<&str> = &Vec::new();
    for i in 0..length {
        if cons {
            cat = rng.gen_range(0..5);
            match cat {
                0 | 1 | 2| 3 => {list = &consonants},
                4 => {list = &c_digraphs},
                _ => {}
            }
            let index = list.len();
            name += list[rng.gen_range(0..index)];
        } else {
            cat = rng.gen_range(0..7);
            match cat {
                0 | 1 | 2| 3 | 4 | 5 => {list = &vowels},
                6 => {list = &v_digraphs},
                _ => {}
            }
            let index = list.len();
            name += list[rng.gen_range(0..index)];
        }
        cons = !cons;
    }
    
    name
}

pub fn gen_room(rng: &mut ThreadRng) {
    let size: Size = Size::new(rng.gen_range(3..13), rng.gen_range(3..13));
}

pub fn process_input<'a>(mut state: &mut String, item_roll: Option<&'a Box<dyn Item>>, inv: &mut Vec<&'a Box<dyn Item>>) -> u8{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read input!");
    input = String::from(input.trim());
    match input.as_str() {
        "z" => {
            pickup(item_roll, state, inv);
            0
        }
        "c"=> {
            println!("\nYou continue down the hallway...\n");
            *state = String::from("walking down");
            0
        },
        "x" => {
            println!("\nJust checking out the floor?\n");
            *state = String::from("standing in");
            0
        },
        // "v" => {
        //     println!("")
        // }
        "q" => {
            we_quit(inv);
            1
        }
        _ => {
            println!("\nWhat are you trying to do?\n");
            *state = String::from("goofing off in");
            0
        }
    }
}

pub fn pickup<'a>(item_roll: Option<&'a Box<dyn Item>>, state: &mut String, inv: &mut Vec<&'a Box<dyn Item>>) {
    if item_roll.is_none() {
        println!("\nThere's nothing to pick up here...\n");
        *state = String::from("goofing off in");
    } else {
        let item = item_roll.unwrap();
        let item_name = String::from(item.as_ref().get_name());
        inv.push(item);
        println!("\nYou put the {0} in your pocket. Losers weepers!\n", item_name);
        *state = String::from("stealing things in");
    }
}

pub fn we_quit<'a>(inv: &mut Vec<&'a Box<dyn Item>>) {
    println!("\nYou quit walking down the hallway and go home. Whew!");
    if inv.len() > 0 {
        println!("What a haul! You got home with:");
        for thing in inv {
            let name = thing.as_ref().get_name();
            if ['a', 'e', 'i', 'o', 'u'].contains(&name.chars().nth(0).unwrap()) {
                print!("\tAn ")
            } else {
                print!("\tA ")
            }
            println!("{0}", thing.as_ref().get_name());
        }
    }
}

pub fn write_to(view_name: &str, output: &String, siv: &mut Cursive) {
    siv.call_on_name(view_name, |v: &mut TextView| {
        v.set_content(output);
    });
}

pub fn load_text() {
    
}
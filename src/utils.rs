use std::io;

use crate::cell::*;

pub fn fetch_coordinate() -> usize {
    let coordinate;

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => { 
                println!("Failed to read line");
                continue;
            }
        }

        match input.trim().parse() {
            Ok(val) => {
                coordinate = val;
            }
            Err(_) => { 
                println!("Please type a number!");
                continue;
            }
        }

        break;
    }

    coordinate
}

pub fn get_cell_index(size: usize, row: usize, col: usize) -> usize {
    return (col - 1) + size * (row - 1);
}

pub fn is_slice_full(slice: &Vec<&Cell>) -> bool {
    let mut is_full = true;

    for i in 0..slice.len() - 1 {
        if slice[i] == &Cell::Empty || slice[i + 1] == &Cell::Empty {
            return false;
        }

        is_full = is_full && slice[i] == slice[i + 1];
    }

    is_full
}
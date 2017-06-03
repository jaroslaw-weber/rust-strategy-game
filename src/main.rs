mod country;
mod gamestate;
mod battle;
mod masterdata;

use std::io::{stdin, stdout, Write};
use std::collections::HashMap;
use country::{Country, Province};

//module: main module

//main function
fn main() {
    //initialize gamestate
    let mut countries = masterdata::load_countries();
    let popped = countries.pop();
    match popped {
        Some(user_country) => {

            let mut gs = gamestate::GameState::new(user_country);
            match countries.pop() {
                Some(c) => gs.add_enemy_country(c),
                None => println!("failed to get enemy country"),
            }

            //loop and read input
            let mut input = String::new();
            //loop after user input a command
            loop {
                //show commands
                gs.show_available_commands();
                println!("________");
                //get input
                input = String::new();
                match stdin().read_line(&mut input) {
                    //react to input
                    Ok(bytes_read) => {
                        gs.execute_command(&input);
                    }
                    Err(error) => println!("error: {}", error),
                }

            }
        } 
        None => println!("failed to get user country"),
    }

}

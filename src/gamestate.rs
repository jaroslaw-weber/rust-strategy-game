use country::{Country, Province};
use battle::{ArmyType, get_army_types_as_str_vec};
use std::fmt;
//module: controling state of the game and handling commands

//functions for GameState
impl GameState {
    //creating new gamestate
    pub fn new(_user_country: Country) -> GameState {
        let new_gs = GameState {
            user_country: _user_country,
            menu_state: MenuState::MainMenu,
            other_countries: Vec::new(),
            make_army_province_selected: String::new(),
            make_army_type_selected: String::new(),
        };
        new_gs
    }

    //show available commands based on current menu
    pub fn show_available_commands(&self) {
        let cmd_list = self.get_available_commands_list();
        println!("available commands: {:?}", cmd_list);
        match self.menu_state {
            MenuState::MainMenu => (),
            _ => println!("to go to main menu write: main menu"),
        }
    }

    //get available commands for current menu
    fn get_available_commands_list(&self) -> Vec<&str> {
        let mut l = vec![];
        match self.menu_state {
            MenuState::MainMenu => l = vec!["wait", "invest", "make army", "attack", "info"],
            MenuState::Investing => l = vec!["Army", "Infrastructure"],
            MenuState::CreatingArmySelectProvince => l = self.user_country.get_provinces_names(),
            MenuState::CreatingArmySelectType => l = get_army_types_as_str_vec(),
            _ => (),
        }
        l
    }

    //add enemy country to world (GameState). should be called just after the initialization
    pub fn add_enemy_country(&mut self, _country: Country) {
        self.other_countries.push(_country);
    }

    //executing command (user input). commands are different in different menus
    pub fn execute_command(&mut self, command: &str) {
        let cmd = command.trim();
        if cmd == "main menu" {
            self.go_to_new_menu(MenuState::MainMenu);
            //return here
        }
        match self.menu_state {
            MenuState::MainMenu => {
                match cmd {
                    "wait" => self.on_end_turn(),
                    "invest" => self.go_to_new_menu(MenuState::Investing),
                    "make army" => self.go_to_new_menu(MenuState::CreatingArmySelectProvince),
                    "info" => self.show_country_status(),
                    _ => println!("command not implemented"),
                }
            }
            MenuState::Investing => self.user_country.invest_in(cmd),
            MenuState::CreatingArmySelectProvince => {

                self.make_army_province_selected = cmd.to_string();
                self.go_to_new_menu(MenuState::CreatingArmySelectType);

            }
            MenuState::CreatingArmySelectType => {
                self.make_army_type_selected = cmd.to_string();
                self.go_to_new_menu(MenuState::CreatingArmySetNumber);
            }
            MenuState::CreatingArmySetNumber => {
                let parsed = cmd.parse();
                match parsed {
                    Ok(n) => {
                        self.user_country
                            .try_make_army(&self.make_army_province_selected,
                                           &self.make_army_type_selected,
                                           n);
                    }
                    Err(e) => println!("failed to parse army number"),
                }
                //go back to main menu after creating army (or failing)
                self.go_to_new_menu(MenuState::MainMenu);
            }
            _ => println!("not implemented"),

        }
    }

    fn go_to_new_menu(&mut self, new_menu: MenuState) {
        self.menu_state = new_menu;
        //need to make copy for this
        println!("menu now: {}", new_menu);
        match new_menu.clone() {
            MenuState::MainMenu => println!("main menu"),
            MenuState::Investing => println!("select investment"),
            MenuState::CreatingArmySelectProvince => println!("select province"),
            MenuState::CreatingArmySelectType => println!("select army type"),
            MenuState::CreatingArmySetNumber => println!("how many army units?"),
            _ => println!("not implemented for {}", new_menu),
        }
        //self.show_available_commands();
    }

    //on end turn, mostly calculating money after collecting taxes
    fn on_end_turn(&mut self) {
        self.user_country.on_turn_end();
        for c in self.other_countries.iter_mut() {
            c.on_turn_end();
        }
        println!("turn ended");
    }

    //show current status of your country
    fn show_country_status(&self) {
        println!("your country status: ");
        println!("{:?}", self.user_country);
    }
}


//struct controlling game state/menu state
pub struct GameState {
    user_country: Country, //player country
    other_countries: Vec<Country>, //other countries
    menu_state: MenuState, //selected menu
    make_army_province_selected: String, //selected province to make army
    make_army_type_selected: String, //selected army type to make
}


//available menus
#[derive(Debug, Copy, Clone)]
pub enum MenuState {
    //basic
    MainMenu,
    Investing,
    //creating army
    CreatingArmySelectType,
    CreatingArmySetNumber,
    CreatingArmySelectProvince,
    //attacking
    AtackingSelectCountry,
    AtackingSelectProvince,
}

impl fmt::Display for MenuState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
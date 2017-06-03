use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;
use std::cmp::{PartialEq, Eq};
use battle::{ArmyType, ArmyUnit, BattleResult, get_army_type_with_string, get_armytype_base_cost};

//module:countries, provinces, etc.

//functions for Country
impl Country {
    //constructor
    pub fn new(_name: &str) -> Country {
        let c = Country {
            name: _name.to_string(),
            money: 0,
            provinces: Vec::new(),
            investment: HashMap::new(),
        };
        c
    }
    //add province to country
    pub fn add_province(&mut self, province: Province) {
        self.provinces.push(province);
    }

    //on turn end (update money etc)
    pub fn on_turn_end(&mut self) {
        self.money += 100;
    }

    //create army. returns success
    pub fn try_make_army(&mut self, province_name: &String, army_type_name: &String, number: i64) {
        let prov_name_copy = province_name.to_string(); //need to dereference to use for comparision
        let army_type_option = get_army_type_with_string(army_type_name);
        match army_type_option {
            Ok(army_type) => {
                let cost = Country::get_army_cost(&army_type, number);
                if self.money >= cost {
                    self.money -= cost;
                    let new_army = ArmyUnit::new(army_type, number);

                    let mut search_province =
                        self.provinces.iter_mut().find(|x| x.name == prov_name_copy);
                    match search_province {
                        Some(prov) => {
                            prov.province_army.push(new_army);
                        }
                        None => println!("province not found"),

                    }
                }
            }
            Err(error) => println!("{}", error),

        }
    }

    //get army cost
    fn get_army_cost(army_type: &ArmyType, number: i64) -> i64 {
        number * get_armytype_base_cost(army_type)
    }



    fn attack_province(&mut self, enemy_province: Province) -> Province {
        enemy_province
    }

    //merging ArmyUnits in whole country
    fn merge_same_type_of_armies(&mut self) {
        //todo
        //creating new army create new ArmyUnit struct, make a option to merge ArmyUnits of same type
    }

    //invest
    pub fn invest_in(&mut self, investition_name: &str) {
        let investition_parsed = get_investment_enum(investition_name);
        match investition_parsed {
            None => println!("investition name invalid!"),
            Some(inv) => {
                if self.investment.contains_key(&inv) {
                    *self.investment.get_mut(&inv).unwrap() += 1;
                } else {
                    self.investment.insert(inv, 1);
                }
                println!("invested in {}", investition_name);
            }
        }

    }
    pub fn get_provinces_names(&self)->Vec<&str>
    {
        self.provinces.iter().map(|x|x.name.as_ref()).collect()
    }
}

//parse string to enum (Investment)
fn get_investment_enum(enum_as_string: &str) -> Option<InvestmentType> {
    let e: Option<InvestmentType>;
    match enum_as_string {
        "Army" => e = Some(InvestmentType::Army),
        "Infrastructure" => e = Some(InvestmentType::Infrastructure),
        _ => e = None,
    }
    e
}

//country
#[derive(Debug)]
pub struct Country {
    money: i64,
    provinces: Vec<Province>,
    name: String,
    investment: HashMap<InvestmentType, i64>,
}

//province (is owned by country)
#[derive(Debug)]
pub struct Province {
    province_army: Vec<ArmyUnit>,
    name: String,
    population: i64,
    monthlyIncome: i64,
}

//province functions
impl Province {
    //constructor
    pub fn new(_name: &str, _population: i64) -> Province {
        let p = Province {
            name: _name.to_string(),
            monthlyIncome: 100,
            population: _population,
            province_army: Vec::new(),
        };
        p
    }
}
//type of investment
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum InvestmentType {
    Army,
    Infrastructure,
}
impl fmt::Display for InvestmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

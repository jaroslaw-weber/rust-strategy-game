//module: calculating battle result and army related structs and functions
use std::fmt;

//result of the battle
pub struct BattleResult {
    won: bool,
    attacker_army: Vec<ArmyUnit>,
    defender_army: Vec<ArmyUnit>,
}

//type of ArmyUnitmenus
#[derive(Debug)]
pub enum ArmyType {
    Farmers,
    Infantry,
    Cavalry,
    Artillery,
}

//display implementation
impl fmt::Display for ArmyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
 
    }
}

//get army types as string vector
pub fn get_army_types_as_str_vec() -> Vec<&'static str> {
    let armt= vec!["Farmers", "Infantry", "Cavalry", "Atrillery"]; //todo iterate through enum
    armt
}



//calculate army strength
pub fn get_army_strenght(army: &Vec<ArmyUnit>) -> i64 {
    let mut strength: i64 = 0;
    for au in army {
        match au.unit_type {
            ArmyType::Farmers => strength += 1,
            ArmyType::Infantry => strength += 2,
            ArmyType::Cavalry => strength += 3,
            ArmyType::Artillery => strength += 4,
            _ => println!("strength not implemented for: {}", au.unit_type), //todo should use panic?
        }
    }
    strength
}

//parse string to ArmyType option
pub fn get_army_type_with_string(enum_as_string: &str) -> Result<ArmyType, String> {
    let mut at: Result<ArmyType, String> = Err("not found".to_string());
    match enum_as_string {
        "Farmers" => at = Ok(ArmyType::Farmers),
        "Infantry" => at = Ok(ArmyType::Infantry),
        "Cavalry" => at = Ok(ArmyType::Cavalry),
        "Artillery" => at = Ok(ArmyType::Artillery),
        _ => at = Err("not found".to_string()),
    }
    at
}


//army (one unit)
#[derive(Debug)]
pub struct ArmyUnit {
    unit_type: ArmyType,
    unit_count: i64,
}

//army unit functions
impl ArmyUnit {
    //constructor
    pub fn new(army_type: ArmyType, number: i64) -> ArmyUnit {
        let na = ArmyUnit {
            unit_type: army_type,
            unit_count: number,
        };
        na
    }
}

//get result of battle
pub fn get_battle_result(attackers_army: Vec<ArmyUnit>,
                         defenders_army: Vec<ArmyUnit>)
                         -> BattleResult {
    let ats = get_army_strenght(&attackers_army);
    let ds = get_army_strenght(&defenders_army);
    let won = ats > ds;
    let result = BattleResult {
        won: won,
        attacker_army: attackers_army,
        defender_army: defenders_army,
    };
    result
}

//how much one unit of ArmyType costs?
pub fn get_armytype_base_cost(army_type: &ArmyType) -> i64 {
    let mut c = 1;
    match *army_type {
        ArmyType::Farmers => c = 1,
        ArmyType::Infantry => c = 2,
        ArmyType::Cavalry => c = 3,
        ArmyType::Artillery => c = 4,
    }
    c
}
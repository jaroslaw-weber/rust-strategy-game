use country::{Country,Province};

//module: loading game data. 

//load all countries
pub fn load_countries() -> Vec<Country>
{
    let mut p=load_provinces();
    let mut c=Vec::new();
    let mut poland = Country::new("Poland");
    let mut germany = Country::new("Germany");
    match p.pop()
    {
        Some(prov)=>poland.add_province(prov),
        None=>println!("error")
    }
    match p.pop()
    {
        Some(prov)=>germany.add_province(prov),
        None=>println!("error")
    }
    c.push(poland);
    c.push(germany);
    c
}

/*
//todo load countries and provinces from csv

//load csv to vector of string vectors
fn load_csv()-> Vec<Vec<String>>
{

}
*/

//load all provinces
fn load_provinces()-> Vec<Province>
{
    let mut p=Vec::new();
    p.push(Province::new("Warsaw", 300));
    p.push(Province::new("Berlin", 300));
    p

}
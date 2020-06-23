#[macro_use(value_t)] extern crate clap;
extern crate serde_json;
extern crate serde;

mod rates;
mod income;

use clap::{Arg, App, ArgMatches};
use rates::*;
use income::{TaxCalculation};

const COUNTRY: &str = "country";
const INCOME: &str = "income";

fn get_matches <'a>() -> ArgMatches<'a> {
    return App::new("tax")
        .version("1.0")
        .about("Calculates progressive income tax")
        .author("James Hay")
        .arg(Arg::with_name(COUNTRY).short("c").long(COUNTRY).takes_value(true))
        .arg(Arg::with_name(INCOME).short("v").long(INCOME).takes_value(true))
        .get_matches(); 
}

fn main() {
    let matches = get_matches();

    let income = value_t!(matches.value_of(INCOME), f32).expect("Something");
    let nz_taxed = get_nz_tax().tax_on_income(income);
    let sg_taxed = get_sg_tax().tax_on_income(income);

    println!("Income {}", income);
    println!("NZ Tax {} - {}", nz_taxed, income - nz_taxed);
    println!("SG Tax {} - {}", sg_taxed, income - sg_taxed);
}


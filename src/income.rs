use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomeBracket {
  pub from: f32,
  pub to: Option<f32>,
  pub tax: f32
}

pub trait TaxCalculation {
    fn tax_on_income (&self, income: f32) -> f32;
}

impl TaxCalculation for IncomeBracket {
  fn tax_on_income (&self, income: f32) -> f32 {
    let to = 
      if let Some(val) = self.to { val }
      else { f32::INFINITY };

    let operate_on = 
      if income <= self.from { 0.0 } 
      else if income <= to { income - self.from } 
      else { to - self.from };

    return operate_on * self.tax / 100.0;
  }
}

impl TaxCalculation for Vec<IncomeBracket> {
  fn tax_on_income (&self, income: f32) -> f32 {
    return self.iter().fold(0.0, |acc, x| acc + x.tax_on_income(income));
  }
}
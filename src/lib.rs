use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
  operation: Operation,
  data: HashMap<String, String>,
}

impl Input {
  pub fn new(operation: Operation, data: HashMap<String, String>) -> Input {
    Input { operation, data }
  }

  pub fn new_from_args(args: &[String]) -> Result<Input, &str> {
    let category = match args.get(1) {
      Some(i) => i.clone(),
      None => return Err("No category (first argument) was provided, while it is required")
      //TODO Show categories available when no category is provided
    };
    let equation = match args.get(2) {
      Some(i) => i.clone(),
      None => return Err("No operation (second argument) was provided, while it is required")
      //TODO Show operations available (in this category) when no operation is provided
    }.to_lowercase();

    let operation = match Operation::new(&category, &equation) {
      Some(i) => i,
      None => return Err("The operation provided is not valid or it is not supported"),
    };

    let data = HashMap::new();
    
    Ok(Input { operation, data })
  }
}

#[derive(Debug)]
pub enum Operation {
  Gravity(Gravity),
  Unknown,
}

impl Operation {
  pub fn new(category: &str, equation: &str) -> Option<Operation> {
    let category = category.to_lowercase();
    let equation = equation.to_lowercase();

    if category == "gravity" || category == "gravitation" {
      if equation == "force" {
        return Some(Operation::Gravity(Gravity::Force));
      } else if equation == "field" {
        return Some(Operation::Gravity(Gravity::Field));
      }
    }

    None
  }
}

#[derive(Debug)]
pub enum Gravity {
  Force,
  Field,
}

impl Gravity {
  pub fn solve(&self, variables: HashMap<&str,&str>) -> Result<&str, &str> {
    match *self {
      Gravity::Force => match Gravity::solve_force(variables) {
        Ok(result) => return Ok(result),
        Err(err) => return Err(err),
      },
      Gravity::Field => match Gravity::solve_field(variables) {
        Ok(result) => return Ok(result),
        Err(err) => return Err(err),
      },
      _ => return Err("The method for solving the provided operation is not yet implemented")
    }
  }

  fn solve_force<'a>(variables: HashMap<&str, &str>) -> Result<&'a str, &'a str>{
    let mass1: String = match variables.get("mass1") {
      Some(i) => i.to_string(),
      None => return Err("'mass1' is required for calculating the Gravitational Force"),
    };
    let mass2: String = match variables.get("mass2") {
      Some(i) => i.to_string(),
      None => return Err("'mass2' is required for calculating the Gravitational Force"),
    };
    let radius: String = match variables.get("radius") {
      Some(i) => i.to_string(),
      None => return Err("'radius' is required for calculating the Gravitational Force"),
    };

    let mass1: u32 = match mass1.trim().parse() {
      Ok(m) => m,
      Err(err) => return Err("Error while parsing mass1"),
    };
    let mass2: u32 = match mass2.trim().parse() {
      Ok(m) => m,
      Err(err) => return Err("Error while parsing mass2"),
    };
    let radius: u32 = match radius.trim().parse() {
      Ok(r) => r,
      Err(err) => return Err("Error while parsing radius"),
    };

    Ok("success")
  }

  fn solve_field<'a>(variables: HashMap<&str, &str>) -> Result<&'a str, &'a str> {
    if true {
      return Err("lol")
    }
    Ok("success")
  }
}

fn run() {}
fn resolve() {} //?
fn resolve_orbits_speed() {} 

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn input_from_args() {
    let data = HashMap::new();
    let operation = Operation::Gravity(Gravity::Field);

    let args: Vec<String> = vec![String::from("."), String::from("gravity"), String::from("force")];

    let base_input = Input::new(operation, data);
    let gen_input = Input::new_from_args(&args).expect("Error while creating input");
    
    assert_eq!(
      base_input.data,
      gen_input.data
    );
    println!("Base operation: {:?}, gen operation: {:?}", base_input.operation, gen_input.operation);
  }
}

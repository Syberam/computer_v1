extern crate regex;
use std::str::FromStr;

pub trait CharExt {
	fn is_equation(self) -> bool;
}

impl CharExt for char {
    #[inline]
    fn is_equation(self) -> bool {
        match self {
            '0'..='9' => true,
            'X' => true,
			'*' | '+' | '-' | '=' | '^' | '.' | '³' | '²' => true,
			_ => false,
        }
    }
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Component{
	pub exponent: i32,
	pub factor: f64,
}

fn	get_delta(components: Vec<Component>) -> f64 {
	let c: f64 = components[0].factor;
	let b: f64 = components[1].factor;
	let a: f64 = components[2].factor;

	b.powi(2) - (4.0 * a * c)
}

fn	solve_second_deg_eq(components: Vec<Component>) -> String {
	let delta: f64 = get_delta(components.clone());
	let mut output: String = format!("∆ :{:?}", delta);
	if delta < 0.0 {
		output = format!(
			"{}\n{}\n{}",
			output,
			"Discriminant is strictly negative, the solution is:",
			"Ø"
		);
	}
	else if delta == 0.0 {
		output = format!(
			"{}\n{}\n{}",
			output,
			"Discriminant is equal to zero, the solution is:",
			(-1.0 * components[1].factor) / (2.0 * components[2].factor)
		);
	}
	else if delta > 0.0 {
		output = format!(
			"{}\n{}\n{}\n{}",
			output,
			"Discriminant is strictly positive, the two solutions are:",
		//(-b - √∆) / 2a	
			((-1.0 * components[1].factor - delta.sqrt()))
			/ (2.0 * components[2].factor),
		//(-b + √∆) / 2a
			((-1.0 * components[1].factor + delta.sqrt()))
			/ (2.0 * components[2].factor)
		);
	}
	output
}

fn solve_first_deg_eq(mut components: Vec<Component>) -> String {
	let mut components_bis: Vec<Component> = components.clone();
	let mut divid: f64 = 0.0;
	let mut diviz: f64 = 0.0;
	components.retain(|&x| x.exponent == 0);
	components_bis.retain(|&x| x.exponent == 1);

	if components.len() > 0 {
		divid = components.first().unwrap().factor;
	}
	if components_bis.len() > 0 {
		diviz = components_bis.first().unwrap().factor;
	}
	if diviz == 0.0 {
		if divid == 0.0 {
			format!("Every ℝéels numbers can be the solution.")
		}
		else {
			format!("No solution is possible.")
		}
	} else {
		let solution: f64 = -divid / diviz;
		format!("The solution is:\n{}", solution)
	}
}

fn solve_zero_deg_eq(components: Vec<Component>) -> String {
	let factor: f64 = components.last().unwrap().factor;
	if factor == 0.0 {
		format!("Every ℝéels numbers can be the solution.")
	}
	else {
		format!("No solution is possible.")
	}
}

fn get_degree(components: Vec<Component>) -> i32 {
	let deg_max: i32 = components.last().unwrap().exponent;
	let deg_min: i32 = components.first().unwrap().exponent;
	if deg_max < 3 && deg_min < 0 {
		return deg_min
	}
	deg_max
}

use regex::Regex;

fn do_poweri(raw_power: &str) -> i32 {
	let re = Regex::new(r"-?\d+\.?\d?\^?").unwrap();
	let mut numbers: Vec<f32> = Vec::new();
	for raw_float in re.captures_iter(raw_power) {
		numbers.push(f32::from_str(&raw_float[0].replace("^", "")).unwrap());
	}
	let mut pow = 1.0;
	for number in numbers.iter().rev() {
        pow = number.powf(pow);
    }
	return pow as i32
}

fn do_powerf(raw_power: &str) -> f64 {
	let re = Regex::new(r"-?\d+\.?\d?\^?").unwrap();
	let mut numbers: Vec<f64> = Vec::new();
	for raw_float in re.captures_iter(raw_power) {
		numbers.push(f64::from_str(&raw_float[0].replace("^", "")).unwrap());
	}
	let mut pow = 1.0;
	for number in numbers.iter().rev() {
        pow = number.powf(pow);
    }
	return pow
}

fn get_components(eq: &str) -> Vec<Component> {
	let mut components: Vec<Component> = vec![
		Component{exponent: 0, factor: 0.0},
		Component{exponent: 1, factor: 0.0},
		Component{exponent: 2, factor: 0.0}];
	//let eq = add_missing_x(eq);
	let eq = eq.replace(char::is_whitespace, "")
		.replace("^+", "^")
		.replace("x", "X")
		.replace("³", "^3")
		.replace("²", "^2")
		.replace("*", "")
		.replace("X", "X^1")
		.replace("X^1^", "X")
		.replace(" + ", "+")
		.replace(" - ", "-")
		.replace("-", "+-")
		.replace("-X", "-1X");
	if !eq.chars().all(CharExt::is_equation) {
		panic!("Entry equation not well format !")
	}
	let sub_strings: Vec<&str> = eq.split("=").collect();
	if sub_strings.len() != 2 {
		panic!("Entry equation not well format !");
	}
	let left_string = match sub_strings[0].chars().nth(0).unwrap() {
			'+' => sub_strings[0].replacen("+", "0+", 1).clone(),
			_ => format!("0+{}",sub_strings[0]),
	};
	let right_string = match sub_strings[1].chars().nth(0).unwrap() {
			'+' => sub_strings[1].replacen("+", "0+", 1).clone(),
			_ => format!("0+{}",sub_strings[1]),
	};
	let left: Vec<&str> = get_substrings(&left_string);
	let right: Vec<&str> = get_substrings(&right_string);
	for elem in left.iter() {
		let elem = match elem.chars().nth(0).unwrap() {
			'X' => elem.replace("X", "1X"),
			_ => elem.to_string(),
		};
		let sub: Vec<&str> = elem.split("X").collect();
		let mut factor: f64 = do_powerf(sub[0]);
		let exponent: i32 = match sub.len() {
			1 => 0,
			_ => do_poweri(sub[1]),
		};
		let mut i:usize = 0;
		for comp in components.iter() {
			if comp.exponent == exponent {
				factor = comp.factor + factor;
				components.remove(i);
				break ;
			}
			i = i + 1;
		}
		let comp: Component = Component{
			exponent: exponent, factor: factor};
		components.push(comp);
	}
	for elem in right.iter(){
		let elem = match elem.chars().nth(0).unwrap() {
			'X' => elem.replace("X", "1X"),
			_ => elem.to_string(),
		};
		let sub: Vec<&str> = elem.split("X").collect();
		let mut factor: f64 = do_powerf(sub[0]);
		let exponent: i32 = match sub.len() {
			1 => 0,
			_ => do_poweri(sub[1]),
		};
		let mut i:usize = 0;
		for comp in components.iter() {
			if comp.exponent == exponent {
				factor = comp.factor - factor;
				components.remove(i);
				break ;
			}
			i = i + 1;
		}
		let comp: Component = Component{
			exponent: exponent, factor: factor};
		components.push(comp);
	}
	components.sort_by(|a, b| a.exponent.cmp(&b.exponent));
	if components.len() <= 3 {
		components.retain(|&x| x.factor != 0.0);
	}
	components
}

pub fn get_substrings(eq: &str) -> Vec<&str> {
	let sub_strings: Vec<&str> = eq.split("+").to_owned().collect();
	sub_strings
}

fn reduce_eq(components: Vec<Component>) -> String {
	let mut reduce_string: String = String::new();
	let mut i = 0;
	for comp in components.iter() {
		if i == 0 {
			reduce_string.push_str(
				&format!("{} * X^{}", comp.factor, comp.exponent)
			);
		}
		else if comp.factor < 0.0 {
			reduce_string.push_str(
				&format!(" - {} * X^{}", comp.factor * -1.0, comp.exponent)
			);
		}
		else {
			reduce_string.push_str(
				&format!(" + {} * X^{}", comp.factor, comp.exponent)
			);
		}
		i = i + 1;
	}
	if components.len() == 0 {
		reduce_string.push_str("0");
	}
	reduce_string.push_str(" = 0".into());
	reduce_string
}

pub fn reduce_from_str(eq: &str) -> String {
	let components: Vec<Component> = get_components(eq);
	reduce_eq(components)
}

pub fn get_eq_degree_from_str(eq: &str) -> i32 {
	let components: Vec<Component> = get_components(eq);
	get_degree(components)
}

pub fn solve_eq(eq: &str) -> Result<String, Box<dyn std::error::Error>>{
	let components: Vec<Component> = get_components(eq);
	let reduce_form = reduce_eq(components.clone());
	let mut output: String = format!("Reduced form: {}", reduce_form);
	if reduce_form == "0 = 0" {
		output = format!(
				"{}\nEvery ℝéels numbers can be the solution.",
				output
		);
		return Ok(output)
	}
	if components.len() == 0 {
		output = format!(
				"{}\nThis is not an equation.",
				output
		);
		return Ok(output)
	}
	let degree:i32 = get_degree(components.clone());
	output = format!(
		"{}\nPolynomial degree: {}",
		output, 
		degree
	);
	if degree > 2 {
		output = format!(
			"{}\n{} {}",
			output,
			"The polynomial degree is",
			"stricly greater than 2, I can't solve."
		)
	}
	else if degree < 0 {
		output = format!(
			"{}\n{} {}",
			output,
			"An exponent is",
			"stricly smaller than 0, I can't solve."
		)
	}
	else if degree == 0 {
		output = format!(
			"{}\n{}",
			output,
			solve_zero_deg_eq(components)
		);
	}
	else if degree == 1 {
		output = format!(
			"{}\n{}",
			output,
			solve_first_deg_eq(components)
		);
	}
	else if degree == 2 {
		output = format!(
			"{}\n{}",
			output,
			solve_second_deg_eq(components)
		);
	}
	Ok(output)
}

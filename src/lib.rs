extern crate regex;

pub mod component;
use component::Component;

mod char_ext;
use char_ext::CharExt;

mod solver;
use solver::solve_zero_deg_eq;
use solver::solve_first_deg_eq;
use solver::solve_second_deg_eq;

pub mod utils;
use utils::do_poweri;
use utils::do_powerf;
use utils::get_degree;

pub fn get_components_from_sub_eq(components: Vec<Component>,
	sub_eq: Vec<&str>, sign: f64) -> 
	Result<Vec<Component>, Box<dyn std::error::Error>> {

	let mut components = components;

	for elem in sub_eq.iter() {
		if elem.matches("X").count() > 1 {
			return Err(format!("{}", "Entry equation not well format !"))?
		}
		let elem = match elem.chars().nth(0) {
			Some('X') => elem.replace("X", "1X"),
			_ => elem.to_string(),
		};
		let sub: Vec<&str> = elem.split("X^").collect();
		let mut factor: f64 = do_powerf(sub[0])?;
		let exponent: i32 = match sub.len() {
			1 => 0,
			_ => do_poweri(sub[1])?,
		};
		let mut i:usize = 0;
		for comp in components.iter() {
			if comp.exponent == exponent {
				factor = comp.factor + (sign * factor);
				components.remove(i);
				break ;
			}
			i = i + 1;
		}
		let comp: Component = Component{
			exponent: exponent, factor: factor};
		components.push(comp);
	}
	Ok(components)
}

pub fn get_components(eq: &str) ->
	Result<Vec<Component>, Box<dyn std::error::Error>> {
	let mut components: Vec<Component> = vec![
		Component{exponent: 0, factor: 0.0},
		Component{exponent: 1, factor: 0.0},
		Component{exponent: 2, factor: 0.0}];
	let mut eq = eq.replace(char::is_whitespace, "")
		.replace("x", "X")
		.replace(" + ", "+")
		.replace(" - ", "-")
		.replace("-", "+-")
		.replace("³", "^3")
		.replace("²", "^2")
		.replace("*", "")
		.replace("X", "X^1")
		.replace("^+", "^")
		.replace("X^1^", "X^")
		.replace("-X", "-1X");
	if !eq.chars().all(CharExt::is_equation) || eq.contains("X^1X") {
		Err(format!("{}", "Entry equation not well format !"))?
	}
	if !eq.contains('X') {
		Err(format!("{}", "This is not an equation !"))?
	}
	if !eq.contains('=') {
		eq = eq + "= 0";
	}
	let sub_strings: Vec<&str> = eq.split("=").collect();
	if sub_strings.len() != 2
		|| sub_strings[0].is_empty() || sub_strings[1].is_empty(){
		Err(format!("{}", "Entry equation not well format !"))?
	}
	let left_string = match sub_strings[0].chars().nth(0) {
			Some('+') => sub_strings[0].replacen("+", "0+", 1).clone(),
			_ => format!("0+{}",sub_strings[0]),
	};
	let right_string = match sub_strings[1].chars().nth(0) {
			Some('+') => sub_strings[1].replacen("+", "0+", 1).clone(),
			_ => format!("0+{}",sub_strings[1]),
	};
	let left: Vec<&str> = left_string.split("+").to_owned().collect();
	let right: Vec<&str> = right_string.split("+").to_owned().collect();
	components = get_components_from_sub_eq(components, left, 1.0)?;
	components = get_components_from_sub_eq(components, right, -1.0)?;
	components.sort_by(|a, b| a.exponent.cmp(&b.exponent));
	Ok(components)
}

pub fn reduce_eq(components: Vec<Component>) ->
	Result<String, Box<dyn std::error::Error>> {
	let mut reduce_string: String = String::new();
	let mut i = 0;
	let mut components = components;

	if components.len() <= 3 {
		components.retain(|&x| x.factor != 0.0);
	}
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
		reduce_string.push_str("X = X");
	}
	else {
		reduce_string.push_str(" = 0".into());
	}
	Ok(reduce_string)
}

pub fn solve_eq(eq: &str) -> Result<String, Box<dyn std::error::Error>> {
	let components: Vec<Component> = get_components(eq)?;
	let reduce_form = reduce_eq(components.clone())?;
	let mut output: String = format!("Reduced form: {}", reduce_form);
	if reduce_form == "X = X" {
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
	let degree:i32 = get_degree(components.clone())?;
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
			solve_second_deg_eq(components)?
		);
	}
	Ok(output)
}

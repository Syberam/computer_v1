use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub struct Component{
	pub exponent: i32,
	pub factor: f64,
}

fn solve_first_deg_eq(components: Vec<Component>) -> () {
	unimplemented!();
}

fn solve_zero_deg_eq(components: Vec<Component>) -> () {
	let factor: f64 = components.last().unwrap().factor;
	if factor == 0.0 {
		println!("Tous les nombres Réels sont solutions.");
	}
	else {
		println!("Aucune solution n'est possible.");
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

fn get_components(eq: &str) -> Vec<Component> {
	let mut components: Vec<Component> = Vec::new();
	let sub_strings: Vec<&str> = eq.split(" = ").collect();
	if sub_strings.len() != 2 {
		println!("not well format");
		return components
	}
	let left: Vec<String> = get_substrings(sub_strings[0]);
	let right: Vec<String> = get_substrings(sub_strings[1]);
	for elem in left.iter(){
		let sub: Vec<&str> = elem.split(" * X^").collect();
		let exponent: i32 = i32::from_str(sub[1]).unwrap();
		let mut factor: f64 = f64::from_str(sub[0]).unwrap();
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
		let sub: Vec<&str> = elem.split(" * X^").collect();
		let exponent: i32 = i32::from_str(sub[1]).unwrap();
		let mut factor: f64 = f64::from_str(sub[0]).unwrap();
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
	components
}

pub fn get_substrings(eq: &str) -> Vec<String> {
	let mut sub_strings: Vec<String> = Vec::new();
	let sub_strings_pos: Vec<&str> = eq.split(" + ").collect();
	for sub_string in sub_strings_pos {
		let sub_strings_neg: Vec<&str> = sub_string.split(" - ").collect();
		if sub_strings_neg.len() == 1 {
			sub_strings.push(sub_string.to_string());
		}
		else {
			let neg: String = "-".to_owned() + sub_strings_neg[1];
			sub_strings.push(sub_strings_neg[0].to_string());
			sub_strings.push(neg.to_string());
		}
	}
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

pub fn solve_eq(eq: &str) -> (){
	let components: Vec<Component> = get_components(eq);
	println!("Reduced form: {}", reduce_eq(components.clone()));
	let degree:i32 = get_degree(components.clone());
	println!("Polynomial degree: {}", degree);
	if degree > 2 {
		println!("{} {}", "The polynomial degree is",
		"stricly greater than 2, I can't solve.")
	}
	else if degree < 0 {
		println!("{} {}", "An exponent is",
		"stricly smaller than 0, I can't solve.")
	}
	else if degree == 0 {
		solve_zero_deg_eq(components);
	}
	else if degree == 1 {
		solve_first_deg_eq(components);
	}
	else if degree == 2 {

	}
}



// $>./computor "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"
// Reduced form: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0
// Polynomial degree: 2
// Discriminant is strictly positive, the two solutions are:
// 0.905239
// -0.475131
// $>./computor "5 * X^0 + 4 * X^1 = 4 * X^0"
// Reduced form: 1 * X^0 + 4 * X^1 = 0
// Polynomial degree: 1
// The solution is:
// -0.25
// ./computor "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0"
// Reduced form: 5 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 0
// Polynomial degree: 3
// The polynomial degree is stricly greater than 2, I can't solve.
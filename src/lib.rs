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

fn	solve_second_deg_eq(components: Vec<Component>) -> () {
	let delta: f64 = get_delta(components.clone());
	println!("∆ :{:?}", delta);	
	if delta < 0.0 {
		println!("Discriminant is strictly negative, the solution is:");
		println!("Ø");
	}
	else if delta == 0.0 {
		println!("Discriminant is strictly positive, the solution is:");
		println!("{}",
		(-1.0 * components[1].factor) / (2.0 * components[2].factor));
	}
	else if delta > 0.0 {
		println!("Discriminant is strictly positive, the two solutions are:");
		//(-b - √∆) / 2a	
		println!("{}",
		((-1.0 * components[1].factor - delta.sqrt()))
		/ (2.0 * components[2].factor));
		//(-b + √∆) / 2a
		println!("{}",
		((-1.0 * components[1].factor + delta.sqrt()))
		/ (2.0 * components[2].factor));
	}
}

fn solve_first_deg_eq(mut components: Vec<Component>) -> () {
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
			println!("Every ℝéels numbers can be the solution.");
		}
		else {
			println!("No solution is possible.");
		}
	} else {
		let solution: f64 = -divid / diviz;
		println!("The solution is:\n{}", solution);
	}
}

fn solve_zero_deg_eq(components: Vec<Component>) -> () {
	let factor: f64 = components.last().unwrap().factor;
	if factor == 0.0 {
		println!("Every ℝéels numbers can be the solution.");
	}
	else {
		println!("No solution is possible.");
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
	let mut components: Vec<Component> = vec![
		Component{exponent: 0, factor: 0.0},
		Component{exponent: 1, factor: 0.0},
		Component{exponent: 2, factor: 0.0}];
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
	println!("simplified eq : {:?}", eq);
	if !eq.chars().all(CharExt::is_equation) {
		println!("Entry equation not well format !");
		components = Vec::new();
		return components
	}
	let sub_strings: Vec<&str> = eq.split("=").collect();
	if sub_strings.len() != 2 {
		println!("not well format");
		return components
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
	println!("LEFT : {:?}", left);
	println!("RIGHT : {:?}", right);
	for elem in left.iter() {
		let elem = match elem.chars().nth(0).unwrap() {
			'X' => elem.replace("X", "1X"),
			_ => elem.to_string(),
		};
		let mut sub: Vec<&str> = elem.split("X").collect();
		println!("SUB :{:?}", sub);
		for mut part in sub {
			let split_part: Vec<&str> = part.split("^").collect();
			part = match split_part.len() {
				1 => split_part[0],
				_ => &f64::from_str(split_part[0])
					.unwrap()
					.powf(f64::from_str(split_part[0]).unwrap())
					.to_string(),
		};
			
		}
		let mut factor: f64 = f64::from_str(sub[0]).unwrap();
		let exponent: i32 = match sub.len() {
			1 => 0,
			_ => i32::from_str(sub[1]).unwrap(),
		};
		println!("exponent : {:?}", exponent);
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
		let mut factor: f64 = f64::from_str(sub[0]).unwrap();
		let exponent: i32 = match sub.len() {
			1 => 0,
			_ => i32::from_str(sub[1]).unwrap(),
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
	println!("components {:?}", components);
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


// TODO: MANAGE IF A == 0 !!
pub fn solve_eq(eq: &str) -> (){
	let components: Vec<Component> = get_components(eq);
	if components.is_empty() {
		return ();
	}
	println!("eq : {:?}", eq);
	let reduce_form = reduce_eq(components.clone());
	println!("Reduced form: {}", reduce_form);
	if reduce_form == "0 = 0" && eq.contains("X") {
		println!("Every ℝéels numbers can be the solution.");
		return ();
	}
	if components.len() == 0 {
		println!("This is not an equation");
		return ();
	}
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
		solve_second_deg_eq(components);
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
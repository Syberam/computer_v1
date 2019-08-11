use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Component{
	pub exponent: u32,
	pub factor: f64,
}

fn get_components(left: Vec<String>, right: Vec<String>) -> Vec<Component> {
	let mut components: Vec<Component> = Vec::new();
	for elem in left.iter(){
		let sub: Vec<&str> = elem.split(" * X^").collect();
		let exponent: u32 = u32::from_str(sub[1]).unwrap();
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
		let exponent: u32 = u32::from_str(sub[1]).unwrap();
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

pub fn reduce(eq: &str) -> String {
	let sub_strings: Vec<&str> = eq.split(" = ").collect();
	if sub_strings.len() != 2 {
		return "not well format".into()
	}
	let left: Vec<String> = get_substrings(sub_strings[0]);
	let right: Vec<String> = get_substrings(sub_strings[1]);
	let mut components: Vec<Component> = get_components(left, right);
	components.sort_by(|a, b| a.exponent.cmp(&b.exponent));
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
	println!("{:?}", reduce_string);
	reduce_string
}
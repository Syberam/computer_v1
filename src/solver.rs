use super::component::Component;

fn	get_delta(components: Vec<Component>) -> f64 {
	let c: f64 = components[0].factor;
	let b: f64 = components[1].factor;
	let a: f64 = components[2].factor;

	b.powi(2) - (4.0 * a * c)
}

pub fn	solve_second_deg_eq(components: Vec<Component>)
	-> Result<String, Box<dyn std::error::Error>> {
	let delta: f64 = get_delta(components.clone());
	let mut output: String = format!("∆: {:?}", delta);
	if delta < 0.0 {
		output = format!(
			"{}\n{}\n{}",
			output,
			"Discriminant is strictly negative, the solution is:",
			"Ø"
		);
	}
	else if delta == 0.0 {
		let sol = (((-1.0 * components[1].factor)
			/ (2.0 * components[2].factor)) * 1_000_000.0).round()
			/ 1_000_000.0;

		output = format!(
			"{}\n{}\n{}",
			output,
			"Discriminant is equal to zero, the solution is:",
			sol
		);
	}
	else if delta > 0.0 {
		//(-b - √∆) / 2a	
		let sol_one = ((((-1.0 * components[1].factor - delta.sqrt()))
			/ (2.0 * components[2].factor)) * 1_000_000.0).round()
			/ 1_000_000.0;
		//(-b + √∆) / 2a
		let sol_two = ((((-1.0 * components[1].factor + delta.sqrt()))
			/ (2.0 * components[2].factor)) * 1_000_000.0).round()
			/ 1_000_000.0;

		output = format!(
			"{}\n{}\n{}\n{}",
			output,
			"Discriminant is strictly positive, the two solutions are:",
			sol_one,
			sol_two
		);
	}
	Ok(output)
}

pub fn solve_first_deg_eq(mut components: Vec<Component>) -> String {
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
		let solution: f64 = ((-divid / diviz) * 1_000_000.0).round()
			/ 1_000_000.0;
		format!("The solution is:\n{}", solution)
	}
}

pub fn solve_zero_deg_eq(components: Vec<Component>) -> String {
	let factor: f64 = components.first().unwrap().factor;
	if factor == 0.0 {
		format!("Every ℝéels numbers can be the solution.")
	}
	else {
		format!("No solution is possible.")
	}
}

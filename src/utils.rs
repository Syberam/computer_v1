use super::component::Component;

pub fn get_degree(components: Vec<Component>) 
	-> Result<i32, Box<dyn std::error::Error>>  {
	let mut components = components;
	components.retain(|&x| x.factor != 0.0);
	let deg_max: i32 = components.last().unwrap().exponent;
	let deg_min: i32 = components.first().unwrap().exponent;

	if deg_max < 3 && deg_min < 0 {
		return Ok(deg_min)
	}
	Ok(deg_max)
}

use regex::Regex;
use std::str::FromStr;

pub fn do_poweri(raw_power: &str) -> i32 {
	let re = Regex::new(r"-?\d+\.?\d*\^?").unwrap();
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

pub fn do_powerf(raw_power: &str) -> f64 {
	let re = Regex::new(r"-?\d+\.?\d*\^?").unwrap();
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

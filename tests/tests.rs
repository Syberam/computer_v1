use computer_v1::component::Component;
use computer_v1::get_components;
use computer_v1::utils::get_degree;
use computer_v1::reduce_eq;

pub fn reduce(eq: &str) -> Result<String, Box<dyn std::error::Error>> {
	let components: Vec<Component> = get_components(eq)?;
	reduce_eq(components)
}

pub fn degree(eq: &str) -> i32 {
	let components: Vec<Component> = get_components(eq).unwrap();
	get_degree(components)
}

#[test]
fn reduce_poly_1deg(){
	assert_eq!("1 * X^0 + 4 * X^1 = 0",
	reduce("5 * X^0 + 4 * X^1 = 4 * X^0").unwrap())
}

#[test]
fn degree_poly_1deg(){
	assert_eq!(1,
	degree("5 * X^0 + 4 * X^1 = 4 * X^0"))
}

#[test]
fn reduce_poly_1deg_2(){
	assert_eq!("-4 * X^0 + 4 * X^1 = 0",
	reduce("4X = 4").unwrap())
}

#[test]
fn degree_poly_1deg_2(){
	assert_eq!(1,
	degree("4X = 4"))
}

#[test]
fn reduce_poly_1deg_3(){
	assert_eq!("4 * X^1 = 0",
	reduce("4X = 0").unwrap())
}

#[test]
fn degree_poly_1deg_3(){
	assert_eq!(1,
	degree("4X = 0"))
}

#[test]
fn degree_poly_2deg(){
	assert_eq!(2,
	degree("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"))
}

#[test]
fn reduce_poly_2deg(){
	assert_eq!("4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0",
	reduce("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0").unwrap())
}

#[test]
fn reduce_poly_2deg_human_write(){
	assert_eq!("4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0",
	reduce("5 + 4x - 9.3x² = 1").unwrap())
}

#[test]
fn reduce_poly_2deg_many_powers(){
	assert_eq!("4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0",
	reduce("5 * X^0^1 + 2^2 * X^0^0 - 9.3 * X^2^1 = 1 * X^0").unwrap())
}

#[test]
fn reduce_poly_3deg(){
	assert_eq!("5 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 0",
	reduce("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0").unwrap())
}


#[test]
fn degree_poly_3deg(){
	assert_eq!(3,
	degree("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0"))
}

#[test]
fn reduce_poly_3deg_2() {
	assert_eq!("7 * X^0 + 0 * X^1 + 0 * X^2 + 2 * X^3 = 0",
	reduce("2X^3+7=0").unwrap())
}

#[test]
fn reduce_poly_neg_deg() {
	assert_eq!("2 * X^-1 + 7 * X^0 + 0 * X^1 + 0 * X^2 = 0",
	reduce("2X^-1+7=0").unwrap())
}

//Errors

#[test]
fn test_no_error() {
	match reduce(" 2 = 2X") {
		Ok(_) => assert!(true),
		Err(_) => assert!(false, "Should not return an error ! "),
	}
}

#[test]
fn test_error_not_an_equation() {
	match reduce(" 2 = 2") {
		Ok(_) => assert!(false,
			"Should return error:\"This is not an equation !\""),
		Err(_) => assert!(true),
	}
}

#[test]
fn test_error_not_an_equation2() {
	match reduce(" = 2") {
		Ok(_) => assert!(false,
			"Should return error:\"This is not an equation !\""),
		Err(_) => assert!(true),
	}
}

#[test]
fn test_error_not_an_equation3() {
	match reduce("J'aime beaucoup de poulet") {
		Ok(_) => assert!(false,
			"Should return error:\"This is not an equation !\""),
		Err(_) => assert!(true),
	}
}

#[test]
fn test_error_not_an_equation4() {
	match reduce("\r\t") {
		Ok(_) => assert!(false,
			"Should return error:\"This is not an equation !\""),
		Err(_) => assert!(true),
	}
}



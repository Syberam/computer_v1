use computer_v1::reduce as reduce;

#[test]
fn reduce_poly_1deg(){
	assert_eq!("1 * X^0 + 4 * X^1 = 0",
	reduce("5 * X^0 + 4 * X^1 = 4 * X^0"))
}

#[test]
fn reduce_poly_2deg(){
	assert_eq!("4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0",
	reduce("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"))
}

#[test]
fn reduce_poly_3deg(){
	assert_eq!("5 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 0",
	reduce("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0"))
}
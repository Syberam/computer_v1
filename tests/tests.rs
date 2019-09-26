use computer_v1::reduce_from_str as reduce;
use computer_v1::get_eq_degree_from_str as degree;

#[test]
fn reduce_poly_1deg(){
	assert_eq!("1 * X^0 + 4 * X^1 = 0",
	reduce("5 * X^0 + 4 * X^1 = 4 * X^0"))
}

#[test]
fn degree_poly_1deg(){
	assert_eq!(1,
	degree("5 * X^0 + 4 * X^1 = 4 * X^0"))
}

#[test]
fn reduce_poly_2deg(){
	assert_eq!("4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0",
	reduce("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"))
}

#[test]
fn degree_poly_2deg(){
	assert_eq!(2,
	degree("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"))
}

#[test]
fn reduce_poly_3deg(){
	assert_eq!("5 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 0",
	reduce("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0"))
}


#[test]
fn degree_poly_3deg(){
	assert_eq!(3,
	degree("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0"))
}

// to test errors ?
// match fs::metadata(path) {
//     Ok(_) => assert!(false, "db file should not exist"),
//     Err(_) => assert!(true),
// }
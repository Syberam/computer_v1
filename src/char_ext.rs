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
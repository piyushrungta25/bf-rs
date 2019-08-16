use bfrs::parser::{Token, parse};

fn test_helper(input: &'static str, output: Vec<Token>) -> Result<(), Box<std::error::Error>> {
    let input = Vec::<u8>::from(input);
    assert_eq!(parse(input), output);
    Ok(())
}

#[test]
fn test_parser_1() -> Result<(), Box<std::error::Error>> {
	test_helper(
		"+++++",
		vec![Token::Inc(5)]
	)
}

#[test]
fn test_parser_2() -> Result<(), Box<std::error::Error>> {
	test_helper(
		"+++++---",
		vec![Token::Inc(2)]
	)
}

#[test]
fn test_parser_3() -> Result<(), Box<std::error::Error>> {
	test_helper(
		"+++++-----",
		vec![]
	)
}

#[test]
fn test_parser_4() -> Result<(), Box<std::error::Error>> {
	test_helper(
		"-----",
		vec![Token::Inc(251)]
	)
}

#[test]
fn test_parser_5() -> Result<(), Box<std::error::Error>> {
	test_helper(
		">>>>>",
		vec![Token::Shift(5)]
	)
}

#[test]
fn test_parser_6() -> Result<(), Box<std::error::Error>> {
	test_helper(
		"<<<<<",
		vec![Token::Shift(-5)]
	)
}

#[test]
fn test_parser_7() -> Result<(), Box<std::error::Error>> {
	test_helper(
		"<<<<<>>>",
		vec![Token::Shift(-2)]
	)
}


#[test]
fn test_parser_8() -> Result<(), Box<std::error::Error>> {
	test_helper(
		"++++[>++++<-]",
		vec![
			Token::Inc(4),
			Token::LoopStart(1, 6),
			Token::Shift(1),
			Token::Inc(4),
			Token::Shift(-1),
			Token::Inc(255),
			Token::LoopEnd(1, 6),

		]
	)
}

#[test]
fn test_parser_9() -> Result<(), Box<std::error::Error>> {
	test_helper(
		"...",
		vec![Token::Write, Token::Write, Token::Write]
	)
}

#[test]
fn test_parser_10() -> Result<(), Box<std::error::Error>> {
	test_helper(
		",,,",
		vec![Token::Read, Token::Read, Token::Read]
	)
}

#[test]
fn test_parser_11() -> Result<(), Box<std::error::Error>> {
	test_helper(
		">>><<<",
		vec![]
	)
}

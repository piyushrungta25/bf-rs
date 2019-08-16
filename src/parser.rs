#[derive(Clone, Debug)]
pub enum Token {
	Inc(u8),
	Shift(i32),
	Read,
	Write,
	LoopStart(usize, usize),
	LoopEnd(usize, usize),
}

pub fn parse(file_contents: Vec<u8>) -> Vec<Token>{
	let mut tokens = Vec::new();
	let mut loop_stack = Vec::<usize>::new();

	for (i, c) in file_contents.iter().enumerate() {
		let c = *c;
		match c {
			b'.' => tokens.push(Token::Write),
			b',' => tokens.push(Token::Read),
			inc if c==b'+' || c==b'-' => {
				let mut inc_count: u8 = 0;
				if let Some(&Token::Inc(n)) = tokens.last() {
					tokens.pop();
					inc_count = n;
				}
				match inc {
					b'+' => {
						let (new, _)  = inc_count.overflowing_add(1);
						inc_count = new;
					},
					_ => {
						let (new, _)  = inc_count.overflowing_sub(1);
						inc_count = new;
					}
				}
				if inc_count != 0 {
					tokens.push(Token::Inc(inc_count));
				}
			},
			shift if c==b'<' || c==b'>' => {
				let mut shift_count: i32 = 0;
				if let Some(&Token::Shift(n)) = tokens.last() {
					tokens.pop();
					shift_count = n;
				}
				match shift {
					b'>' => shift_count += 1,
					_ => shift_count -= 1
				}
				if shift_count != 0 {
					tokens.push(Token::Shift(shift_count));
				}
			},
			b'[' => {
				loop_stack.push(tokens.len());
				tokens.push(Token::LoopStart(tokens.len(), 0));
			},
			b']' => {
				let loop_start_index = loop_stack.pop().expect(&format!("Opening brace not found for closing brace at {}", i));
				if let Some(&Token::LoopStart(start, _end)) = tokens.get(loop_start_index) {
					tokens[loop_start_index] = Token::LoopStart(start, tokens.len());
					tokens.push(Token::LoopEnd(start, tokens.len()));
				} else {
					panic!("Unexpected error occured");
				}

			},
			_ => {}
		}
	}

	if let Some(opening_brace_index) = loop_stack.pop() {
		if let Some(&Token::LoopStart(start, _end)) = tokens.get(opening_brace_index) {
			panic!("Closing brace for opening brace at {} not found", start);
		}

	}
	tokens
}
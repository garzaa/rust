// given an input string of contents, returns a vector of strings
// that will to live as long as the contents
// so we're going to be returning string slices or something? yes
// without this, Rust compiler sees we're returning a borrowed string
// but doesn't know which input it'll be borrowed from
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results: Vec<&str> = Vec::new();
	// could use iterators...but later
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results: Vec<&str> = Vec::new();
	// could use iterators...but later
	for line in contents.lines() {
		if line.to_lowercase().contains(query) {
			results.push(line);
		}
	}

	results
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
unga bunga.
me unga.
duct tape";
		assert_eq!(vec!["duct tape"], search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "duct";
		let contents = "\
hungus
Duct tape
		";
		assert_eq!(vec!["Duct tape"], search_case_insensitive(query, contents));
	}
}

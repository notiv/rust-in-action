fn greet_world() {
	println!("Hello, world!");
	let switzerland = "Gruezi!";
	let greece = "Γεια σας!";
	let regions = [switzerland, greece];
	for region in regions.iter() {
		println!("{}", &region);
	}	
}

fn main() {
	greet_world();
}


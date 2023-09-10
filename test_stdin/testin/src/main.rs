use std::io::{self, Read};

fn main() -> io::Result<()> {

let mut buffer = String::new();
 let stdin = match io::stdin().read_to_string(&mut buffer) {
	 Ok(o) => o,
	Err(e) => { println!("Stdin error: {}", e); return Err(e); }
 };  
 print!("{}",stdin);
 Ok(())

}

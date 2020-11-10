use std::process::Command;
//M to the O
fn main() {
    let output = Command::new("zsh")
	.arg("-c")
	.arg("python3 o.py")
	.output()
	.expect("failure to lawnch");
	
}

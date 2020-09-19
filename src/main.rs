use std::fs::File;
use std::env;
use std::io::Read;
use std::path::Path;
fn main() {
	let  args:Vec<String>=env::args().collect();
	if args.len() < 2{
		println!("syntax error : <filepath> not found.\nfar requires filepath to open file.\n\n\tsyntax: far <filepath>\n\teg. far my_files/file.txt");
		return
	}
	let filepath= Path::new(&args[1]);
	let mut file=match File::open(filepath){
		Ok(file) =>file,
		Err(_) =>{
			println!("Could not open the file {:#?}",filepath );
			return 
		}
	};
	let mut content=String::new();
	 match file.read_to_string(&mut content){
		Ok(_) => show_content(content),
		Err(_) =>println!("Sorry!! far could not read the content of file {:#?}" , filepath )
	};
	pub fn show_content(content:String){
		for i in content.chars(){
			print!("{}",i );
		}
	}
}

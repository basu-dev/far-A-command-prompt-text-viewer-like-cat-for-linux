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
	if !args[1].contains(&".") {
		println!("far: {:?}: is a directory.",args[1]);
		return
	}
	let filepath= Path::new(&args[1]);
	let mut file=match File::open(filepath){
		Ok(file) =>file,
		Err(_) =>{
			println!("far: {:?} : File not found!!",filepath );
			return 
		}
	};
	let mut content=String::new();
	 match file.read_to_string(&mut content){
		Ok(_) => show_content(content),
		Err(_) =>println!("far: Sorry!! far could not read the content of file {:#?}" , filepath )
	};
	pub fn show_content(content:String){
		let mut count=1;
		let content_len=content.len();
		println!("\n");
		for i in content.chars(){
			if count==1{
				print!("1  | ");
				count+=1;

			}
			print!("{}",i );
			if i=='\n'{
				if count>=10 &&count <100{
					print!("{} | ",count.to_string() );

				}
				else if count >= 100{
					print!("{}| ",count.to_string() );	
				}

				else {
					print!("{}  | ",count.to_string() );
				}	
				
				count+=1;
			}
		}
		println!("\n\n");
	}

}

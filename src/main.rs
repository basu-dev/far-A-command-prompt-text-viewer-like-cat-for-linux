
use std::fs::File;
use std::env;
use std::io::Read;
use std::path::Path;

fn main() {
    static mut COPY: bool = false;
    let  args:Vec<String>=env::args().collect();
    if args.len() < 2{
        println!("syntax error : <filepath> not found.");
        println!("far requires filepath to open file.");
        print_syntax();
        return
    }
    else if args.len() > 2{
        if args[2]=="--c" || args[2]=="copy" || args[2]=="--copy"{
                unsafe{COPY=true};
        }
    }
    fn print_syntax(){
        println!("\n\tsyntax: far <filepath> < --c | --copy | copy>");
        println!("\teg. far my_files/file.txt");
        println!("\nadding --c flag removes line count, useful for copying the text.");
        println!("\t eg. far my_files/file.txt --c");
    }
    fn check_if_dir(args:&Vec<String>,filepath:&str){
     if !args[1].contains(&".") {
        println!("far: {:?}: is a not a file.",args[1]);
        print_syntax();
        return
        }
    else {
        println!("far: {:?} : File not found!!",filepath );
        return
    }
    };
    let filepath= Path::new(&args[1]);
    let mut file=match File::open(filepath){
        Ok(file) =>file,
        Err(_) =>{
                check_if_dir(& args,filepath.to_str().unwrap());
                return
        }
    };
    let mut content=String::new();
     match file.read_to_string(&mut content){
        Ok(_) => show_content(content,&filepath),
        Err(_) =>println!("far: Sorry!! far could not read the content of file {:#?}" , filepath )
    };
    pub fn show_content(content:String,filepath:&std::path::Path){
        let mut count=1;
        println!("\n");
        unsafe{
        for i in content.chars(){
            if count==1 && COPY==false{
                    print!("1  | ");
                    count+=1;

            }
            print!("{}",i );
            if i=='\n' && COPY==false{
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
    }
            println!("\n try: far {} --c to remove line count for copying file.",filepath.to_str().unwrap());
    }
}





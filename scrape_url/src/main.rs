use std::{fs, env};

// fn main() {
//    let url = "https://www.rust-lang.org/"; 
//     let output = "rust.md"; 
    
//     println!("Fetching url: {}", url); 
//     let body = reqwest::blocking::get(url).unwrap().text().unwrap(); 
//     println!("Converting html to markdown..."); 
//     let md = html2md::parse_html(&body); 
//     fs::write(output, md.as_bytes()).unwrap(); 
//     println!("Converted markdown has been saved in {}.", output);
// }
// fn main()->Result<(),Box<dyn std::error::Error>> {
//    let url="https://www.rust-lang.org/";
//    let output="rust.md";

//    println!("Fetching url: {}",url);
//    let body =reqwest::blocking::get(url)?.text()?;

//    println!("Converting html to markdown...");
//    let md= html2md::parse_html(&body);
//    fs::write(output, md.as_bytes())?;
//    println!("Converted markdown has been saved in {}.",output);

//    Ok(())
// }
fn main() {
   let args:Vec<String>=env::args().collect();
   for (index, value) in args.iter().enumerate() {
        println!("{} => {}", index,value );
    }
   let url=&args[1];
   let output=&args[2];

   println!("Fetching url: {}", url); 
   let body = reqwest::blocking::get(url).unwrap().text().unwrap(); 
   println!("Converting html to markdown..."); 
   let md = html2md::parse_html(&body); 
   fs::write(output, md.as_bytes()).unwrap(); 
   println!("Converted markdown has been saved in {}.", output);
}


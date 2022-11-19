use std::path::Path;


fn main(){
    
    let path =Path::new("12.jpg");
    let ex = path.extension().unwrap().to_str().unwrap().to_owned();
    println!("{}",ex);
}
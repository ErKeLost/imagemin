use std::path::Path;


fn main(){
    
    let path =Path::new("pink.jpeg");
    let ex = path.extension().unwrap().to_str().unwrap().to_owned();
    println!("{}",ex);
}

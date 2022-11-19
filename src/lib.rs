

pub mod img;

use clap::Parser;
use comfy_table::Table;
use comfy_table::Row;
use indicatif::{HumanBytes,HumanDuration,ProgressBar,ProgressStyle};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;
use std::process::exit;
use std::thread;
use std::time::Duration;



#[derive(Parser,Debug)]
#[clap(author = "记事本",version = "0.1")]
pub struct Args{
    pub path:String,
    pub deep:u32
}

#[derive(Debug)]
pub struct Imageh{
    pub path:String,
    pub name:String,
    pub size:u64,
}

impl Imageh{
    fn new(name:&String)->Self{
        Self{
           path:name.clone(),name:"".into(),size:0 
        }
    }
}
impl Default for Imageh{
    fn default() -> Self {
        Self { path: "".into(), name: "".into(), size: 0  }
    }
}

#[derive(Debug)]
pub struct ImageInfo{
    pub prev:Imageh,
    pub later:Imageh,
    pub img_type:String
}

impl ImageInfo {
    fn new(prev_path:&String) ->Self{
       Self { prev: Imageh::new(prev_path), later: Imageh::default(),img_type:"".into() } 
    }
}

pub fn scan(path:&Path,pb:ProgressBar)->Result<Vec<ImageInfo>,Box<dyn Error>>{
    let mut list =vec![];
    if !path.is_dir(){
        eprintln!("please enter true path");
        exit(1);
    }
    let entries = fs::read_dir(path)?;
    for entry in entries{
        let entry =entry?;
        let path = entry.path();
        pb.set_message(path.to_str().unwrap().to_owned());
        pb.inc(1);
        thread::sleep(Duration::from_millis(50));
        if path.is_file(){
            if let Some(extension) =path.extension(){
                let extension = extension.to_str().unwrap().to_owned();
                if extension == "png" || extension == "jpg" || extension == "PNG" || extension == "JPG"{
                    let mut image =ImageInfo::new(&path.to_str().unwrap().to_owned());
                    image.prev.size= path.metadata().unwrap().len();
                    image.prev.name = path.file_name().unwrap().to_str().unwrap().to_owned();
                    image.img_type = extension.to_lowercase().clone();
                    list.push(image);
                }
            }
        }
    }
    Ok(list)
}

pub fn run(config:&Args) ->Result<(),Box<dyn Error>>{

     let pb =  ProgressBar::new_spinner();
     let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");
    pb.set_style(spinner_style);
    pb.set_prefix("Scanning ");

    let started = Instant::now();
    let path = Path::new(&config.path);
    match scan(path, pb.clone()){
        Ok(mut list) => {
            img::img_run(config, &mut list,pb.clone())?;
            print_table(&mut list);
        },
        Err(e) => {
            eprintln!("{}",e.to_string());
        }
    }
    pb.finish_and_clear();
    println!("Finished in {}",HumanDuration(started.elapsed()));
    Ok(())
}


fn print_table(list:&mut Vec<ImageInfo>){

    let mut rows:Vec<Row>= vec![];
    rows.push(
        Row::from(vec![
            "前图片路径".to_owned(),"前图片容量".to_owned(),"后图片路径".to_owned(),"后图片容量".to_owned()
        ])
    );
    for v in list.iter(){
        let row =Row::from( [  v.prev.path.clone(),fomramtSize(v.prev.size)  ,v.later.path.clone(),fomramtSize(v.later.size) ]);
        rows.push(row);

    }
    let mut table = Table::new();
    table.add_rows(rows);

    println!("{table}");
}

fn fomramtSize(filesize:u64) ->String{

    if filesize < 1024 {
        return format!("{}B",filesize);
    }else if filesize < (1024 * 1024){
        return format!("{}KB",filesize / 1024u64);
    }else if filesize < (1024 * 1024 * 1024){
        return  format!("{}MB",filesize / (1024*1024));
    }else if filesize < (1024 * 1024 * 1024 *1024){
        return  format!("{}GB",filesize / (1024* 1024*1024));
    }else {
        return format!("0B");
    }
}
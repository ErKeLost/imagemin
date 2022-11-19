
use super::ImageInfo;
use image::GenericImageView;
use image::imageops::FilterType;
use image::ImageFormat;
use super::Args;
use std::fs;
use std::path::Path;
use std::fs::File;
use indicatif::ProgressBar;


pub fn img_run(args:&Args,list:&mut Vec<ImageInfo>,pb:ProgressBar) ->Result<(),Box<dyn std::error::Error>>{
    pb.set_prefix("being processed");
    let itemPath = format!("{}item",args.path);
    fs::create_dir(&itemPath)?;

    let deep = (args.deep as f32) / 10f32; 
    for imageinfo in list{
       let img = image::open(&imageinfo.prev.path)?; 
       let (width,height) =img.dimensions();
       let new_width = ((width as f32) * deep) as u32;
       let new_height= ((height as f32) * deep) as u32;
       let new_path = format!("{}/{}",itemPath,imageinfo.prev.name);
       imageinfo.later.path= new_path.clone();
       let scaled = img.resize(new_width, new_height, FilterType::Nearest);
       let mut output = File::create(&new_path).unwrap();
       pb.set_message(new_path.clone());
       if imageinfo.img_type == "png"{
        scaled.write_to(&mut output, ImageFormat::Png)?;
       }else if imageinfo.img_type == "jpg"{
        scaled.write_to(&mut output, ImageFormat::Jpeg)?;
       }
       imageinfo.later.size = file_size(&new_path)?;
    }
    Ok(())
}

fn file_size(path:&String) ->Result<u64,Box<dyn std::error::Error>>{
    let path = Path::new(path);
    let m =path.metadata()?;
    Ok(m.len())
}
use std::fs;

fn main() -> std::io::Result<()> {
    fs::remove_dir_all("./../output")?;
    Ok(())
}

// fn main() {
    // let dir = fs::read_dir("./../.output").unwrap();
    //
    // let files: Vec<_> = dir.collect();
    //
    // println!("{:?}", files.len());
    //
    // for file in files {
    //     let _fname = file.unwrap().file_name();
    //     let fname = _fname.to_str().unwrap();
    //     // println!("{}", fname);
    //     let path = format!("./../.output/{}", fname);
    //     let _ = fs::remove_file(path).is_ok();
    //     // println!("{}", result);
    // }
    //
    // let dir = fs::read_dir("./../output").unwrap();
    //
    // let files: Vec<_> = dir.collect();
    //
    // println!("{:?}", files.len());
    //
    // let result = fs::remove_dir_all("./../output").is_ok();
    //
    // println!("{}", result);
// }

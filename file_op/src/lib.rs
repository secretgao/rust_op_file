use std::error;
use std::io::{self,Read,Write,BufReader, BufRead};
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::collections::BTreeMap; 
//打开文件读取
pub fn open_read_file(filename:&str)->Result<String,std::io::Error> {
    let path = Path::new(filename);
    let f = File::open(&path);
    let mut f = match f {
        Ok(f)=>f,
        Err(e)=> match e.kind() {
            std::io::ErrorKind::NotFound => match File::create("aa.txt") {
                Ok(ff)=>ff,
                Err(ffe)=>panic!("panic :{}",ffe),
            }
            std::io::ErrorKind::PermissionDenied=>panic!("no permission"),
            //其他错误
            some_error => panic!("error :{}",some_error),
        }
    };
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

//打开文件写入 
pub fn open_wirte_file(filename:&str,context:&[u8])->Result<(), Box<dyn error::Error>>{
    let path = Path::new(filename);
    let mut file = OpenOptions::new()
         .read(true)
         .write(true)
         .append(true)
         .create(true)
         .open(path).unwrap();

    file.write(context)?;
    file.flush();
    Ok(())

}
//按行读取
pub fn open_read_file_by_lines(filename:&str)->BTreeMap<i32, String>{
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    let mut read_file_content = BufReader::new(file);
    //定义一个map
   // let mut file_content = HashMap::new();   
    let mut file_content :BTreeMap<i32, String>= BTreeMap::new();
    let mut k = 0;
    for line in read_file_content.lines()  {
        //存入map v = 1，2，3，
        file_content.insert(k, line.unwrap());
        k=k+1;
    }
    return file_content;
}

use std::collections::BTreeMap; 
use std::error;
use std::io::{self,Read,Write,BufReader, BufRead};
//翻转map 每行字符串 返回
//pub fn reverse_map_line_string(map:HashMap<i32, Result<String, io::Error>>)->HashMap<i32, Result<String, io::Error>>{

pub fn reverse_map_line_string(map:BTreeMap<i32, String>)->BTreeMap<i32, String>{

   let len = map.len();
   
   let mut result  :BTreeMap<i32, String>= BTreeMap::new();
    for (k,v) in &map  {
        let vv:Vec<&str> = v.split(",").collect();
        let snew: Vec<&str> = vv.into_iter().rev().collect();
        let mut tmp = String::new();
        for i in snew{
            if  i !=""{
                tmp = tmp + &i.to_string() + ",";
            }
        }
        result.insert(len as i32 - k, tmp);
    
    }
    return result;
}

/*
&str -> String--| String::from(s) or s.to_string() or s.to_owned()
&str -> &[u8]---| s.as_bytes()
&str -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()
String -> &str----| &s if possible* else s.as_str()
String -> &[u8]---| s.as_bytes()
String -> Vec<u8>-| s.into_bytes()
&[u8] -> &str----| std::str::from_utf8(s).unwrap()
&[u8] -> String--| String::from_utf8(s).unwrap()
&[u8] -> Vec<u8>-| s.to_vec()
Vec<u8> -> &str----| std::str::from_utf8(&s).unwrap()
Vec<u8> -> String--| String::from_utf8(s).unwrap()
Vec<u8> -> &[u8]---| &s if possible* else s.as_slice()
 */

use file_op;
use until;
fn main() {
    // 定义写入文件名称
    let filename = "a.txt";
    let other_filename = "b.txt";
    //1创建文件
    //2写入 1-99 
    println!("开始执行");
    for i  in 1..100{  //
        let num = i%3;  //能被3整出 余0 换行
        let conext = i.to_string()+","; 
        file_op::open_wirte_file(filename, conext.as_bytes());
        if num == 0{
            //追加换行
            file_op::open_wirte_file(filename, "\n".as_bytes()); 
        }
    }
    println!("写入文件：{},内容1-99 成功",filename);
    //3读出1-99
    let line_content_map =file_op::open_read_file_by_lines(filename);
    println!("读取文件：{},内容1-99 成功",filename);
   //把集合map中的元素翻转
    let result = until::reverse_map_line_string(line_content_map);
    println!("读取文件：{},内容1-99 反转成功",filename);
    //4写入另一个文件
    for (k,v) in result {
        file_op::open_wirte_file(other_filename, v.as_bytes());
        file_op::open_wirte_file(other_filename, "\n".as_bytes());
    }
    println!("反转1-99内容写入文件：{}",other_filename);
}

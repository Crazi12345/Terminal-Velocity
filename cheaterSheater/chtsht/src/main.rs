use std::fs::File;
use std::os::unix::process::CommandExt;
use std::process::Command;
fn main() {

    let args: Vec<String> = std::env::args().collect();
    let lang: &str = args.get(1).expect("only had one");
//    let option: &str = args.get(2).expect("");
    let result = Command::new("curl").args(["cht.sh/",stringify!("{}",lang)]).exec();
    println!("{}",result);

}

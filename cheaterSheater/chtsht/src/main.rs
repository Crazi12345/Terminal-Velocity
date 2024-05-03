use std::fs::File;
use std::os::unix::process::CommandExt;
use std::process::Command;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lang: &str = args.get(1).expect("only had one");
    let option = args.get(2);
    if !option.is_none() {
        let search = option.unwrap();
        let result = Command::new("curl")
            .args(["cht.sh/".to_owned() + lang + "/" + search])
            .exec();
        let run = Command::new(result.to_string()+"> command.txt").exec();
        println!("{}", result);
        println!("{}", run);
    } else {
        let result = Command::new("curl")
            .args(["cht.sh/".to_owned() + lang])
            .exec();

        println!("{}", result);
    }
}

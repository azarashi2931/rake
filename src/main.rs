use std::env;
use std::fs;
use std::process;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_file = "Makefile";
    let mut work_dir = env::current_dir().unwrap();
    loop {
        let path = {
            let mut tmp = work_dir.clone();
            tmp.push(target_file);
            tmp
        };
        let metadata = fs::metadata(path.to_str().unwrap());
        let exists = match &metadata {
            Ok(file) => file.is_file(),
            Err(_) => false,
        };

        if exists || !work_dir.pop() {
            break;
        }
    }

    let mut make_process = Command::new("make")
        .args(&args[1..])
        .current_dir(work_dir)
        .spawn()
        .expect("failed to run Make");
    process::exit(match make_process.wait() {
        Result::Ok(status) => match status.code() {
            Some(code) => code,
            None => 0,
        },
        Result::Err(_) => panic!(),
    })
}

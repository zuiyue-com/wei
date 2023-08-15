use std::env;

fn main() {
    wei_env::bin_init("wei");

    let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     help();
    //     std::process::exit(1);
    // }
    let mut command = "";

    if args.len() > 1 {
        command = &args[1];
    }

    match command {
        "run" => {
            // 先去当前目录bin下面找对应的exe文件，如果没有，则去wei_env::dir_bin下面找对应执行的路径
            // 如果还是没有，则去网络上面查找有没有对应的exe文件，如果有则去下载。并提示当前正在下载文件
            // 如果在网络上面没有找到对应的exe文件，则提示失败
            let data = run(&args[2], std::env::args().skip(3).collect()).unwrap();
            println!("{}", data);
        },
        "daemon" => {
            println!("daemon");
        },
        "--help" => {
            help();
        },
        _ => {
            
        }
    }
}

fn help() {
    let args: Vec<String> = env::args().collect();
    println!("Usage:");
    println!("  {} run <command> <param1> <param2>", args[0]);
    // println!("  {} api <json>", args[0]);
}

fn run(command: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let path = std::path::Path::new("./bin/").join(command).join(".exe");
    if path.exists() {
        return wei_run::run(&path.display().to_string(), param);
    }

    let path = wei_env::read(&wei_env::dir_bin(),command)?;
    wei_run::run(path.as_str(), param)
}
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::bin_init("wei");
    wei_daemon::start().await.unwrap();

    // todo set exit file to false;

    let args: Vec<String> = env::args().collect();
    let mut command = "";

    if args.len() > 1 {
        command = &args[1];
    }

    match command {
        "run" => {
            let data = wei_run::run(&args[2], std::env::args().skip(3).collect()).unwrap();
            print!("{}", data);
        },
        "daemon" => {
            print!("daemon");
        },
        "--help" => {
            help();
        },
        _ => {
            
        }
    }

    // todo check exit file is true,then exit;

    Ok(())
}

fn help() {
    let args: Vec<String> = env::args().collect();
    println!("Usage:");
    println!("  {} run <command> <param1> <param2>", args[0]);
    // println!("  {} api <json>", args[0]);
}

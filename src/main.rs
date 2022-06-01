use clap::{Parser, Subcommand};
use std::fs;
use std::process::{Child, Command};

fn update_server() {
    let install_path = std::env::current_dir().unwrap().join("ark_server");
    let mut command = Command::new("steamcmd");
    command.args([
        "+force_install_dir",
        install_path.to_str().unwrap(),
        "+login",
        "anonymous",
        "+app_update",
        "376030",
        "+quit",
    ]);
    println!("{:?}", command);
    let status = command.spawn().unwrap().wait().unwrap();
    if !status.success() {
        if let Some(code) = status.code() {
            println!("update failed with code {}", code);
        } else {
            println!("update failed with no status code given")
        }
    }
}

fn run_server(map_name: &str, num: usize, cmd_config: &str) -> Child {
    let port = 7777 + num * 2;
    let query_port = 27015 + num;

    let main_arg = format!("{map_name}?SessionName=BoyScouts{map_name}?AltSaveDirectoryName=Save{map_name}?Port={port}?QueryPort={query_port}?listen?{cmd_config}");

    let working_dir = std::env::current_dir()
        .unwrap()
        .join("ark_server/ShooterGame/Binaries/Linux");
    let mut command = Command::new(working_dir.join("ShooterGameServer"));
    command.current_dir(working_dir).arg(main_arg).args([
        "-server",
        "-log",
        "-ForceRespawnDinos",
        "-NoTransferFromFiltering",
        "-clusterid=boyscouts727",
        "-crossplay",
        "-high",
    ]);
    println!("{:?}", command);

    command.spawn().unwrap()
}

fn run_servers() {
    let cmd_config = if let Ok(data) = fs::read_to_string("./CMDConfig.ini") {
        data.trim().lines().collect::<Vec<_>>().join("?")
    } else {
        println!("could not read CMDConfig.ini");
        String::new()
    };

    let servers = [
        "TheIsland",
        "ScorchedEarth_P",
        "Aberration_P",
        "Extinction",
        "Genesis",
        "Gen2",
        "LostIsland",
    ];
    let mut children: Vec<Child> = servers
        .iter()
        .enumerate()
        .map(|(i, map_name)| run_server(map_name, i, &cmd_config))
        .collect();
    children.iter_mut().for_each(|child| {
        let _ = child.wait().unwrap();
    });
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Run,
    Update,
}

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Run => run_servers(),
        Action::Update => update_server(),
    }
}

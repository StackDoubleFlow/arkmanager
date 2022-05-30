use clap::{Parser, Subcommand};
use std::process::{Command, Child};

const CMD_CONFIG: &[&str] = &[
    "AllowFlyerCarryPvE=true",
    "OverrideOfficialDifficulty=5.0",
    "PreventSpawnAnimations=false",
    "PvEAllowStructuresAtSupplyDrops=true",
    "ShowFloatingDamageText=true",
    "GameModIds=731604991,1522327484",
    "AllowCaveBuildingPVE=true",
    "IgnoreLimitMaxStructuresInRangeTypeFlag=false",
];

fn run_server(map_name: &str, num: usize) -> Child {
    let port = 7777 + num * 2;
    let query_port = 27015 + num;

    let cmd_config = CMD_CONFIG.join("?");
    let main_arg = format!("{map_name}?SessionName=BoyScouts{map_name}?AltSaveDirectoryName=Save{map_name}?Port={port}?QueryPort={query_port}?listen?{cmd_config}");

    let working_dir = std::env::current_dir().unwrap().join("ark_server/ShooterGame/Binaries/Linux");
    let mut command = Command::new(working_dir.join("ShooterGameServer"));
    command.current_dir(working_dir)
        .arg(main_arg)
        .args([
            "-server",
            "-log",
            "-ForceRespawnDinos",
            "-NoTransferFromFiltering",
            "-clusterid=cluster1",
            "-crossplay",
            "-high",
        ]);
    println!("{:?}", command);

    command.spawn().unwrap()
}

fn run_servers() {
    let servers = ["TheIsland", "ScorchedEarth_P", "Aberration_P", "Extinction", "Genesis", "Gen2", "LostIsland"];
    let children: Vec<Child> = servers.iter().enumerate().map(|(i, map_name)| run_server(map_name, i)).collect();
}

#[derive(Parser)]
struct Args {
   #[clap(subcommand)]
   action: Action,
}

#[derive(Subcommand)]
enum Action {
    Run,
    Update
}

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Run => run_servers(),
        Action::Update => todo!(),
    }
}

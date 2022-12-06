use std::{io::Write, str::FromStr};

pub mod algos;
pub mod block;
pub mod cmd;
pub mod memory;

fn main() {
    let path = "test";
    let in_path = format!("{}.in", path);
    let data = std::fs::read_to_string(in_path).unwrap();
    let cmds = cmd::CmdVec::from_str(&data).unwrap();
    let fns: Vec<algos::Algo> = vec![algos::first_fit, algos::best_fit, algos::worst_fit];
    let mut partitions: Vec<memory::Memory> = vec![];
    for algo in fns.iter() {
        let mut memory = memory::Memory::new(cmds.size);
        for cmd in cmds.iter() {
            memory.exec(cmd, algo.clone(), path);
        }
        partitions.push(memory);
    }

    let mut file = std::fs::File::create(format!("{}.out", path)).unwrap();
    let out = partitions
        .iter()
        .map(|p| p.output())
        .collect::<Vec<String>>()
        .join("");

    file.write_all(out.as_bytes()).unwrap();
}

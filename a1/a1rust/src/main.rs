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
    let fns: Vec<(&str, algos::Algo)> = vec![
        ("FirstFit", algos::first_fit),
        ("BestFit", algos::best_fit),
        ("WorstFit", algos::worst_fit),
    ];
    let mut partitions: Vec<(&str, memory::Memory)> = vec![];
    let mut memory = memory::Memory::new(cmds.size);
    for (name, algo) in fns.iter() {
        let mut mem = memory.with_out_count();
        for cmd in cmds.iter() {
            mem.exec(cmd, (*name, algo.clone()), path);
        }
        memory = mem.with_out_count();

        partitions.push((name, mem));
    }

    let mut file = std::fs::File::create(format!("{}.out", path)).unwrap();
    let out = partitions
        .iter()
        .map(|(n, p)| p.output(n))
        .collect::<Vec<String>>()
        .join("");

    file.write_all(out.as_bytes()).unwrap();
}

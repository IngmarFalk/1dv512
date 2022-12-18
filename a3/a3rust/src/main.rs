use a3rust::{
    algorithms::{self, Algorithm},
    simulation::{Data, Simulation},
};

fn main() {
    let raw = std::fs::read_to_string("test.in").unwrap();
    let data = raw.parse::<Data>().unwrap();
    let algorithms: Vec<(&str, Algorithm)> = vec![
        ("fcfs", algorithms::fcfs),
        ("scan", algorithms::scan),
        ("cscan", algorithms::cscan),
    ];

    let mut out = String::new();

    for (name, algorithm) in algorithms {
        let simulation = Simulation {
            algorithm,
            data: data.clone(),
        };

        let res = simulation.run();

        // Append to file
        out.push_str(&format!("{}\n{}\n{:?}\n", name, res.0, res.1));

        println!("{}\n{}\n{:?}", name, res.0, res.1);
    }

    std::fs::write("test.out", out).unwrap();
}

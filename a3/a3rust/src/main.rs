use a3rust::simulation::Data;

fn main() {
    let data = std::fs::read_to_string("input.in").unwrap();
    let simulation = data.parse::<Data>().unwrap();

    println!("{:?}", simulation);
}

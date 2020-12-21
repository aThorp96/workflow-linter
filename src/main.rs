use std::io::Read;

mod custom_types;
mod workflow;

fn main() {
    println!("Imagine having two write this with marshmallow.py");
    let mut file = std::fs::File::open("./test_input/example_issue.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let workflow: workflow::Workflow = serde_yaml::from_str(&contents).unwrap();
    print!("{:?}", workflow);
}

use std::process::Command;

fn main() {
    println!("Hello, world!");
    let mut args = std::env::args().skip(1);

    let mut res = Vec::new();
    // hello world
    while let Some(name) = args.next() {
        res.push(name);
    }

    // hello-world
    let res = res.join("-");

    let output = Command::new("cargo").arg("new").arg(&res).output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Project '{}' created successfully!", res);
            } else {
                println!("Error creating project '{}'", res);
            }
        }
        Err(e) => {
            println!("Error executing command: {}", e);
        }
    }
}

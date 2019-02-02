use std::io;

fn main() {
    use std::collections::HashMap;

    let mut company = HashMap::new();

    let mut name = String::from("Sally");
    let mut department = String::from("Engineering");
    company.insert(name, department);
    name = String::from("Amir");
    department = String::from("Sales");
    company.insert(name, department);
    loop {
        println!("Please input your name:");
        let mut n = String::new();
        io::stdin().read_line(&mut n)
            .expect("Failed to read line");
        n = n.trim().to_string();
        if n == "quit" {
            break;
        }
        println!("Please input your department:");
        let mut depart = String::new();
        io::stdin().read_line(&mut depart)
            .expect("Failed to read line");
        depart = depart.trim().to_string();
        if depart == "quit"{
            break;
        }
        company.insert(n, depart);
    }

    println!("{:?}", company);
    println!("Hello, world!");
}

enum Employee {
    Manager {
        name: String,
        subordinates: Vec<Employee>,
    },
    Worker {
        name: String,
        manager: String,
    },
}

fn main() {
    let bob = Employee::Worker {
        name: "Bob".to_string(),
        manager: "Alice".to_string(),
    };

    let alice = Employee::Manager {
        name: "Alice".to_string(),
        subordinates: vec![(bob)],
    };

    match alice {
        Employee::Manager { name, subordinates } => {
            println!("{} has {} subordinates", name, subordinates.len());
        }
        Employee::Worker { name, manager } => {
            println!("{} reports to {}", name, manager);
        }
    }
}


struct Ticket {
    title: String,
    description: String,
    status: String
}

impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }

    fn new(title: String, description: String, status: String) -> Self {
        let valid_statuses = ["To-DO", "IN PROGRESS", "DONE"];
        if !valid_statuses.contains(&status.as_str()) {
            panic!("Invalid status");
        }

        if title.len() <= 0 {
            panic!("Title should not be empty");
        }
        if title.len() > 50 {
            panic!("Title should not exceed 50 characters");
        }

        if description.len() <= 0 {
            panic!("Description should not be empty");
        }
        if description.len() > 500 {
            panic!("Description should not exceed 500 characters");
        }

        Self {
            title,
            description,
            status
        }
    }
}

struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    // default is a static method
    fn default() -> Configuration {
        Configuration {
            version: 0,
            active: false
        }
    }
}

struct Order {
    price: u32,
    quantity: u32
}

impl Order {
    fn is_available(self) -> bool {
        self.quantity > 0
    }
}


fn main() {
    let ticket1 = Ticket {
        title: "Build a ticket system".into(),
        description: "A Kanban board".into(),
        status: "Open".into()
    };

    let is_open_1 = ticket1.is_open();
    println!("Is the ticket open?\n{}", is_open_1);

    let ticket2 = Ticket {
        title: "Build a ticket system".into(),
        description: "A Kanban board".into(),
        status: "Closed".into()
    };

    let is_open_2 = Ticket::is_open(ticket2);
    println!("Is the ticket open?\n{}", is_open_2);
    is_open_2;

    let default_config = Configuration::default();

}

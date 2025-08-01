mod helpers {
    // TODO: Make this code compile, either by adding a `use` statement or by using
    //  the appropriate path to refer to the `Ticket` struct.
    use crate::Ticket;
    pub fn create_todo_ticket(title: String, description: String) -> Ticket {  // pub = public
        Ticket::new(title, description, "To-Do".into())
    }
}

mod ticket {

}
struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}


fn main() {

    let ticket = helpers::create_todo_ticket(
        String::from("Build a ticket system"),
        "Ticket description".to_string()
    );
    println!("Ticket created: {} - {} - {}", ticket.title, ticket.status, ticket.description);
}

//
// rustreview06-structs    main.rs
//

struct NameRec {
    first_name : String,
    last_name  : String,
    uid        : u32
}

impl NameRec {
    fn new() -> Self {
        Self {
            first_name : String::from("Unknown"),
            last_name  : String::from("Unknown"),
            uid        : 0
        }
    }

    // Getter methods
    fn get_first(&self) -> String {
        self.first_name.clone()
    }

    fn get_last(&self) -> String {
        self.last_name.clone()
    }


    // Setter methods
    fn set_first(&mut self, first : &str) {
        self.first_name = String::from(first);
    }

    fn set_last(&mut self, last : &str) {
        self.last_name = String::from(last);
    }
}

fn bar() {
    println!("\n*******************************************\n");
}

fn main() {
    bar();

    let mut name = NameRec::new();
    println!("last = { }, first = { } : uid = { }", 
              name.get_last(), name.get_first(), name.uid);

    bar();

    name.set_first("Homer");
    name.set_last("Simpson");

    println!("last = { }, first = { } : uid = { }", 
              name.get_last(), name.get_first(), name.uid);

    bar();
}

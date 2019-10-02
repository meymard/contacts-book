pub struct Contact {
    id: i32,
    firstname: String,
    lastname: String,
    phones: [Phone; 0],
}

impl Contact {
    pub fn new() -> Contact {
        Contact {
            id: 0,
            firstname: String::new(),
            lastname: String::new(),
            phones: [],
        }
    }
    pub fn get_firstname(&self) -> String {
        let firstname = self.firstname.clone();

        firstname
    }

    pub fn set_firstname(&mut self, firstname: String) -> &Contact {
        self.firstname = firstname;

        self
    }

    pub fn to_string(&self) -> String {
        let mut contact = "Firstname : ".to_string();
        contact.push_str(self.firstname.as_str());

        contact
    }
}

pub struct Phone {
    pub number: String,
    pub title: String,
}

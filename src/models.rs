pub struct Contact {
    id: i32,
    firstname: String,
    lastname: String,
    phones: Vec<Phone>,
}

impl Contact {
    pub fn new() -> Contact {
        Contact {
            id: 0,
            firstname: String::new(),
            lastname: String::new(),
            phones: Vec::new(),
        }
    }

    pub fn get_id(&self) -> i32 {
        let id = self.id.clone();

        id
    }

    pub fn get_firstname(&self) -> String {
        let firstname = self.firstname.clone();

        firstname
    }

    pub fn set_firstname(&mut self, firstname: String) -> &Contact {
        self.firstname = firstname;

        self
    }

    pub fn get_lastname(&self) -> String {
        let lastname = self.lastname.clone();

        lastname
    }

    pub fn set_lastname(&mut self, lastname: String) -> &Contact {
        self.lastname = lastname;

        self
    }

    pub fn get_phones(&self) -> &Vec<Phone> {
        &self.phones
    }

    pub fn add_phone(&mut self, phone: Phone) -> &Contact {
        self.phones.push(phone);

        self
    }

    pub fn remove_phone(&mut self, phone: Phone) {
        let index: usize = self.phones.iter()
            .enumerate()
            .find(|&p| p.1 == &phone).unwrap().0;

        self.phones.remove(index);
    }
}

impl PartialEq for Contact {
    fn eq(&self, other: &Self) -> bool {
        self.firstname == other.firstname && self.lastname == other.lastname
    }
}

impl ToString for Contact {
    fn to_string(&self) -> String {
        let mut contact = "Firstname : ".to_string();
        contact.push_str(self.firstname.as_str());

        contact
    }
}


pub enum NumberType {
    Mobile,
    Work,
    Home,
    Main,
    Fax,
    Custom,
}

pub struct Phone {
    number: String,
    number_type: NumberType,
    custom_type: String,
}

impl Phone {
    pub fn get_number(&self) -> String {
        let number = self.number.clone();

        number
    }

    pub fn set_number(&mut self, number: String) -> &Phone {
        self.number = number;

        self
    }

    pub fn get_number_type(&self) -> &NumberType {
        let number_type = &self.number_type;

        number_type
    }

    pub fn get_custom_type(&self) -> String {
        let number = self.custom_type.clone();

        number
    }

    pub fn set_custom_type(&mut self, custom_type: String) -> &Phone {
        self.custom_type = custom_type;

        self
    }

    pub fn get_number_type_label(number_type: NumberType) -> String {
        match number_type {
            NumberType::Mobile => "Mobile".to_string(),
            NumberType::Work => "Work".to_string(),
            NumberType::Home => "Home".to_string(),
            NumberType::Main => "Main".to_string(),
            NumberType::Fax => "Fax".to_string(),
            NumberType::Custom => "Custom".to_string(),
        }
    }
}

impl PartialEq for Phone {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

/*impl ToString for Phone {
    fn to_string(&self) -> String {
        let mut contact = Phone::get_number_type_label(&self.number_type);
        contact.push_str(self.number.as_str());

        contact
    }
}*/

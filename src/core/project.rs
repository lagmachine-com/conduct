pub struct Project {
    identifier: String,
    display_name: String,
}

impl Project {
    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn get_display_name(&self) -> String {
        self.display_name.clone()
    }
}

pub fn create_project() -> Project {
    Project {
        identifier: "proj".to_owned(),
        display_name: "My Project".to_owned(),
    }
}

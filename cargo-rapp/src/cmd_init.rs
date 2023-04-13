pub struct Init {
    pub(crate) path: Option<String>,
}

impl Init {
    pub fn run(&mut self) {
        let path = self.path.clone().unwrap_or(".".to_string());
        println!("Git checkout init project and name it '{}'", path)
    }
}

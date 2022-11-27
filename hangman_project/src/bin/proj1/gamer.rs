#[derive(Clone)]
pub struct Gamer {
    name: String,
}

pub fn login(name: &str) -> Gamer {
    Gamer{name: name.to_string()}
}


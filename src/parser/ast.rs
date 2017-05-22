pub enum Statement {
    Symbol(Box<String>)
}

pub type Module = Vec<Statement>;

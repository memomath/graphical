enum Grammar {
    add,
    subtract,
    divide,
    multiply,
    exponent
}

struct ParseNode {
    children: Vec<ParseNode>,
    entry: Grammar,
}
pub mod parser {
    pub fn parse() {

    }
}
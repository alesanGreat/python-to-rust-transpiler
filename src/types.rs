#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Keyword,
    Identifier,
    Number,
    String,
    Symbol,
    Comment,
    Eol,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub tipo: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawCall {
    pub draw_type: DrawType,
    pub x: String,
    pub y: String,
    pub w: Option<String>,
    pub h: Option<String>,
    pub radius: Option<String>,
    pub color: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawType {
    Rect,
    Circle,
}

#[derive(Debug)]
pub struct ControlFlow {
    pub control_type: String,
    pub condition: Option<String>,
}

#[derive(Debug)]
pub struct State {
    pub variables: Vec<Variable>,
    pub draw_calls: Vec<DrawCall>,
    pub control_flow: Vec<ControlFlow>,
}

use crate::types::{Token, TokenType, State, Variable, DrawCall, DrawType, ControlFlow};

#[derive(Debug, PartialEq, Clone)]
enum ASTNode {
    Program(Vec<ASTNode>),
    Assignment { name: String, value: Box<ASTNode> },
    Number(i32),
    Boolean(bool),
    Identifier(String),
    StringLiteral(String),
    BinaryOp { op: String, left: Box<ASTNode>, right: Box<ASTNode> },
    SDL2Call { function: String, args: Vec<ASTNode> },
    DrawCall(DrawCall),
    ControlFlow { control_type: String, condition: Option<Box<ASTNode>>, body: Vec<ASTNode> },
    Ignore,
}

pub fn extract_state(tokens: Vec<Token>) -> State {
    let mut parser = Parser { tokens, current: 0 };
    let ast = parser.parse_program();

    let mut state = State {
        variables: Vec::new(),
        draw_calls: Vec::new(),
        control_flow: Vec::new(),
    };
    extract_state_from_ast(&ast, &mut state);
    
    state
}

fn extract_state_from_ast(node: &ASTNode, state: &mut State) {
    match node {
        ASTNode::Program(statements) => {
            for statement in statements {
                extract_state_from_ast(statement, state);
            }
        }
        ASTNode::Assignment { name, value } => {
            let value_str = format!("{:?}", value);
            let tipo = if value_str == "Boolean(true)" || value_str == "Boolean(false)" {
                "bool".to_string()
            } else if value_str.contains(',') {
                "tuple".to_string()
            } else {
                "i32".to_string()
            };
            state.variables.push(Variable {
                name: name.clone(),
                value: value_str.clone(),
                tipo,
            });
        }
        ASTNode::DrawCall(draw_call) => {
            state.draw_calls.push(draw_call.clone());
        }
        ASTNode::ControlFlow { control_type, condition, body } => {
            state.control_flow.push(ControlFlow {
                control_type: control_type.clone(),
                condition: condition.as_ref().map(|c| format!("{:?}", c)),
            });
            for statement in body {
                extract_state_from_ast(statement, state);
            }
        }
        _ => {}
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn parse_program(&mut self) -> ASTNode {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.try_parse_statement() {
                statements.push(stmt);
            }
        }
        ASTNode::Program(statements)
    }

    fn try_parse_statement(&mut self) -> Option<ASTNode> {
        if self.is_at_end() || self.check(TokenType::Eol, "\n") {
            self.advance();
            return None;
        }

        Some(self.parse_statement())
    }

    fn parse_statement(&mut self) -> ASTNode {
        let token = self.peek();
        match &token.token_type {
            TokenType::Identifier => {
                if self.peek_next().token_type == TokenType::Symbol && self.peek_next().value == "=" {
                    self.parse_assignment()
                } else {
                    self.parse_expression()
                }
            },
            TokenType::Keyword => {
                if self.match_keyword("while") || self.match_keyword("if") {
                    self.parse_control_flow()
                } else {
                    self.parse_expression()
                }
            }
            _ => self.parse_expression()
        }
    }

    fn parse_assignment(&mut self) -> ASTNode {
        let name = self.consume(TokenType::Identifier, "Expected identifier").value;
        self.consume(TokenType::Symbol, "Expected '='");
        let value = self.parse_expression();
        ASTNode::Assignment { name, value: Box::new(value) }
    }

    fn parse_expression(&mut self) -> ASTNode {
        self.parse_binary_expression()
    }

    fn parse_binary_expression(&mut self) -> ASTNode {
        let mut left = self.parse_primary();

        while self.is_binary_operator() {
            let operator = self.advance().value;
            let right = self.parse_primary();
            left = ASTNode::BinaryOp {
                op: operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn is_binary_operator(&self) -> bool {
        if self.is_at_end() {
            return false;
        }
        let token = self.peek();
        token.token_type == TokenType::Symbol && 
        ["+", "-", "*", "/", "//"].contains(&token.value.as_str())
    }

    fn parse_primary(&mut self) -> ASTNode {
        if self.is_at_end() {
            return ASTNode::Ignore;
        }

        let token = self.peek();
        match &token.token_type {
            TokenType::Number => {
                let value = self.advance().value.parse::<i32>().unwrap_or(0);
                ASTNode::Number(value)
            },
            TokenType::Identifier => {
                let value = self.advance().value.clone();
                if !self.is_at_end() && self.check(TokenType::Symbol, ".") {
                    self.current -= 1; // Retroceder para procesar la llamada completa
                    self.parse_sdl2_call()
                } else {
                    ASTNode::Identifier(value)
                }
            },
            TokenType::String => {
                let value = self.advance().value.clone();
                ASTNode::StringLiteral(value)
            },
            TokenType::Keyword => {
                if self.match_keyword("true") {
                    ASTNode::Boolean(true)
                } else if self.match_keyword("false") {
                    ASTNode::Boolean(false)
                } else if self.peek().value.starts_with("sdl2") {
                    self.parse_sdl2_call()
                } else {
                    self.advance();
                    ASTNode::Identifier(String::from("undefined"))
                }
            },
            _ => {
                self.advance();
                ASTNode::Ignore
            }
        }
    }

    fn parse_sdl2_call(&mut self) -> ASTNode {
        let mut function_path = String::new();
        
        // Consumir todos los identificadores y puntos hasta llegar a la función
        while !self.is_at_end() && (self.check(TokenType::Identifier, "") || self.check(TokenType::Symbol, ".") || 
              self.check(TokenType::Keyword, "")) {
            let token = self.advance();
            function_path.push_str(&token.value);
        }

        // Si no hay una función SDL2 válida, retornar Ignore
        if !function_path.contains("sdl2") {
            return ASTNode::Ignore;
        }

        // Manejar los diferentes tipos de llamadas SDL2
        let last_part = function_path.split('.').last().unwrap_or("");
        match last_part {
            "rect" | "fill" => self.parse_sdl2_rect_or_fill_call(function_path),
            "draw_point" => self.parse_sdl2_draw_point_call(),
            "Window" | "Renderer" | "init" | "quit" | "get_events" | "show" | "clear" | "present" =>
                ASTNode::SDL2Call { function: function_path, args: Vec::new() },
            _ => ASTNode::SDL2Call { function: function_path, args: Vec::new() }
        }
    }

    fn parse_sdl2_rect_or_fill_call(&mut self, function: String) -> ASTNode {
        if !self.check(TokenType::Symbol, "(") {
            return ASTNode::SDL2Call { function, args: Vec::new() };
        }

        self.consume(TokenType::Symbol, "Expected '('");
        let mut args = Vec::new();

        // Parsear argumentos hasta encontrar el paréntesis de cierre
        while !self.is_at_end() && !self.check(TokenType::Symbol, ")") {
            args.push(self.parse_expression());
            if self.check(TokenType::Symbol, ",") {
                self.advance();
            }
        }

        self.consume(TokenType::Symbol, "Expected ')'");

        // Crear DrawCall si tenemos suficientes argumentos
        if args.len() >= 4 {
            let draw_call = DrawCall {
                draw_type: DrawType::Rect,
                x: format!("{:?}", args[1]),
                y: format!("{:?}", args[2]),
                w: Some(format!("{:?}", args[3])),
                h: if args.len() > 4 { Some(format!("{:?}", args[4])) } else { Some(String::from("50")) },
                radius: None,
                color: format!("{:?}", args[0]),
            };
            ASTNode::DrawCall(draw_call)
        } else {
            ASTNode::SDL2Call { function, args }
        }
    }

    fn parse_sdl2_draw_point_call(&mut self) -> ASTNode {
        self.consume(TokenType::Symbol, "Expected '('");
        let mut args = Vec::new();

        while !self.is_at_end() && !self.check(TokenType::Symbol, ")") {
            args.push(self.parse_expression());
            if self.check(TokenType::Symbol, ",") {
                self.advance();
            }
        }

        self.consume(TokenType::Symbol, "Expected ')'");

        if args.len() >= 3 {
            let draw_call = DrawCall {
                draw_type: DrawType::Circle,
                x: format!("{:?}", args[1]),
                y: format!("{:?}", args[2]),
                w: None,
                h: None,
                radius: Some(String::from("50")),
                color: format!("{:?}", args[0]),
            };
            ASTNode::DrawCall(draw_call)
        } else {
            ASTNode::SDL2Call { 
                function: String::from("draw_point"), 
                args
            }
        }
    }

    fn parse_control_flow(&mut self) -> ASTNode {
        let control_type = self.peek_previous().value.clone();
        let mut condition = None;

        if self.check(TokenType::Symbol, "(") {
            self.advance();
            condition = Some(Box::new(self.parse_expression()));
            self.consume(TokenType::Symbol, "Expected ')'");
        }

        let mut body = Vec::new();
        while !self.is_at_end() && !self.check(TokenType::Keyword, "end") {
            if let Some(stmt) = self.try_parse_statement() {
                body.push(stmt);
            }
        }

        ASTNode::ControlFlow { 
            control_type, 
            condition, 
            body 
        }
    }

    fn consume(&mut self, expected_type: TokenType, message: &str) -> Token {
        if self.check(expected_type.clone(), "") {
            self.advance()
        } else {
            panic!("Parser Error: Expected {:?}, found {:?} - {}", 
                  expected_type, self.peek().token_type, message);
        }
    }

    fn check(&self, token_type: TokenType, value: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        if !value.is_empty() {
            self.peek().token_type == token_type && self.peek().value == value
        } else {
            self.peek().token_type == token_type
        }
    }

    fn match_keyword(&mut self, value: &str) -> bool {
        if self.check(TokenType::Keyword, value) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.peek_previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Token {
        if self.is_at_end() {
            Token { 
                token_type: TokenType::Eol, 
                value: "\n".to_string() 
            }
        } else {
            self.tokens[self.current].clone()
        }
    }

    fn peek_next(&self) -> Token {
        if self.current + 1 >= self.tokens.len() {
            Token { 
                token_type: TokenType::Eol, 
                value: "\n".to_string() 
            }
        } else {
            self.tokens[self.current + 1].clone()
        }
    }

    fn peek_previous(&self) -> Token {
        if self.current == 0 {
            self.tokens[0].clone()
        } else {
            self.tokens[self.current - 1].clone()
        }
    }
}
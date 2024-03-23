
#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    // Tokens de control
    ENDFILE,
    ERROR,

    // Palabras reservadas
    IF,
    ELSE,
    DO,
    WHILE,
    SWITCH,
    CASE,
    END,
    REPEAT,
    UNTIL,
    READ,
    WRITE,
    INTEGER,
    DOUBLE,
    MAIN,
    AND,
    OR,
    RETURN,

    // Tokens de múltiples caracteres
    ID,
    NumInt,
    NumReal,

    // Operadores aritméticos
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    MODULO,
    POWER,

    // Operadores relacionales
    EQ,   // igualdad
    NEQ,  // diferente
    LT,   // menor que
    LTE,  // menor o igual que
    GT,   // mayor que
    GTE,  // mayor o igual que

    // Símbolos especiales
    LPAREN,    // paréntesis izquierdo
    RPAREN,    // paréntesis derecho
    LBRACE,    // llave izquierda
    RBRACE,    // llave derecha
    COMMA,     // coma
    SEMICOLON, // punto y coma
    ASSIGN,    // asignación

    // Símbolo de comentario múltiple no cerrado
    InMultipleComment,
}

// Enum para representar los estados en el DFA del escáner
#[derive(Debug, PartialEq)]
enum StateType {
    Start,
    InAssign,
    InComment,
    InMultiComment,
    InNum,
    InReal,
    InId,
    Done,
    EndFile,
}

// Función para obtener el siguiente carácter no en blanco de la línea actual
fn get_next_char(line: &str, linepos: &mut usize, bufsize: usize) -> char {
    if *linepos >= bufsize {
        '\0' // Devuelve un carácter nulo al final de la línea
    } else {
        let c = line.chars().nth(*linepos).unwrap_or('\0'); // Usa unwrap_or para devolver un carácter nulo si el índice está fuera de rango
        *linepos += 1;
        c
    }
}

// Función para retroceder un carácter en la línea actual
fn unget_next_char(linepos: &mut usize) {
    if *linepos > 0 {
        *linepos -= 1;
    }
}

// Función para buscar palabras reservadas y devolver su TokenType correspondiente
fn reserved_lookup(s: &str) -> TokenType {
    match s {
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "do" => TokenType::DO,
        "while" => TokenType::WHILE,
        "switch" => TokenType::SWITCH,
        "case" => TokenType::CASE,
        "end" => TokenType::END,
        "repeat" => TokenType::REPEAT,
        "until" => TokenType::UNTIL,
        "read" => TokenType::READ,
        "write" => TokenType::WRITE,
        "int" => TokenType::INTEGER,
        "double" => TokenType::DOUBLE,
        "main" => TokenType::MAIN,
        "return" => TokenType::RETURN,
        "/*" => TokenType::InMultipleComment,
        _ => TokenType::ID,
    }
}

// Función para realizar el análisis léxico y devolver los tokens
fn get_token(content: &str) -> Vec<(TokenType, String, usize, usize)> {
    let mut tokens = Vec::new();
    let mut lineno = 0;
    let mut state = StateType::Start;
    let mut token_string = String::new();
    let mut linepos = 0;
    let bufsize = content.len();
    let mut column_number = 1;
    while linepos <= bufsize {
        column_number +=1;
        let c = get_next_char(content, &mut linepos, bufsize);
        match state {
            StateType::Start => {
                if c == '\n' {
                    lineno += 1;
                    column_number = 1;
                }
                if c.is_whitespace() {
                    // Ignorar espacios en blanco
                } else if c.is_ascii_alphabetic() {
                    state = StateType::InId;
                    token_string.push(c);
                } else if c.is_digit(10) {
                    state = StateType::InNum;
                    token_string.push(c);
                } else if c == '/' {
                    let next_char = get_next_char(content, &mut linepos, bufsize);
                    if next_char == '/' {
                        state = StateType::InComment;
                    } else if next_char == '*' {
                        state = StateType::InMultiComment;
                    } else {
                        tokens.push((TokenType::DIVIDE, "/".to_string(), lineno, column_number));
                        unget_next_char(&mut linepos);
                    }
                } else {
                    match c {
                        '=' => tokens.push((TokenType::EQ, "=".to_string(), lineno, column_number)),
                        '<' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::LTE, "<=".to_string(), lineno, column_number));
                            } else {
                                tokens.push((TokenType::LT, "<".to_string(), lineno, column_number));
                                unget_next_char(&mut linepos);
                            }
                        }
                        '>' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::GTE, ">=".to_string(), lineno, column_number));
                            } else {
                                tokens.push((TokenType::GT, ">".to_string(), lineno, column_number));
                                unget_next_char(&mut linepos);
                            }
                        }
                        '+' => tokens.push((TokenType::PLUS, "+".to_string(), lineno, column_number)),
                        '-' => tokens.push((TokenType::MINUS, "-".to_string(), lineno, column_number)),
                        '*' => tokens.push((TokenType::TIMES, "*".to_string(), lineno, column_number)),
                        '%' => tokens.push((TokenType::MODULO, "%".to_string(), lineno, column_number)),
                        '^' => tokens.push((TokenType::POWER, "^".to_string(), lineno, column_number)),
                        '(' => tokens.push((TokenType::LPAREN, "(".to_string(), lineno, column_number)),
                        ')' => tokens.push((TokenType::RPAREN, ")".to_string(), lineno, column_number)),
                        '{' => tokens.push((TokenType::LBRACE, "{".to_string(), lineno, column_number)),
                        '}' => tokens.push((TokenType::RBRACE, "}".to_string(), lineno, column_number)),
                        ',' => tokens.push((TokenType::COMMA, ",".to_string(), lineno, column_number)),
                        ';' => tokens.push((TokenType::SEMICOLON, ";".to_string(), lineno, column_number)),
                        '&' => tokens.push((TokenType::AND, "&".to_string(), lineno, column_number)),
                        '|' => tokens.push((TokenType::OR, "|".to_string(), lineno, column_number)),
                        ':' => tokens.push((TokenType::ASSIGN, ":".to_string(), lineno, column_number)),
                        '\0' => {
                            state = StateType::EndFile;
                        }
                        _ => tokens.push((TokenType::ERROR, c.to_string(), lineno, column_number)),
                    }
                }
            }
            StateType::InId => {
                if c.is_ascii_alphanumeric() || c == '_' {
                    token_string.push(c);
                } else {
                    tokens.push((reserved_lookup(&token_string), token_string.clone(), lineno, column_number));
                    token_string.clear();
                    state = StateType::Start;
                    continue; // Seguir con el siguiente carácter en la misma iteración
                }
            }
            StateType::InNum => {
                if c.is_digit(10) || c == '.' {
                    token_string.push(c);
                } else {
                    tokens.push((TokenType::NumInt, token_string.clone(), lineno, column_number));
                    token_string.clear();
                    state = StateType::Start;
                    continue; // Seguir con el siguiente carácter en la misma iteración
                }
            }
            StateType::InComment => {
                if c == '\n' || c == '\0' {
                    state = StateType::Start;
                }
            }
            StateType::InMultiComment => {
                if c == '*' {
                    let next_char = get_next_char(content, &mut linepos, bufsize);
                    if next_char == '/' {
                        state = StateType::Start;
                    } else {
                        unget_next_char(&mut linepos);
                    }
                } else if c == '\0' {
                    tokens.push((TokenType::InMultipleComment, "/*".to_string(), lineno, column_number));
                    println!("Error: '/*' Multiline comment not closed.");
                    state = StateType::EndFile;
                }
            }
            StateType::EndFile => {
                tokens.push((TokenType::ENDFILE, "\0".to_string(), lineno, column_number));
                break; // Salir del bucle mientras
            }
            _ => (),
        }
    }
    tokens
}

fn main() {
    let file_content = r#"
    // Ejemplo de código fuente
    int main () { 
            int x1 = 5 ;
            double y = 3.14^2 ;
            ¿
            if (x > 0 & y < 5.0 % 2) {
                x = x * 2 ;
                y = y / 2 ;
            } else { 
                x = x - 1 ;
                y = y + 1 ;
            } 
            return x ;
        }
    "#;
    
    let tokens = get_token(&file_content);
    for (token_type, lexeme, line, column) in tokens {
                println!(
                    "Token: {:?}, Lexema: {}, Linea: {}, Columna: {}",
                    token_type, lexeme, line, column
                );
            }
}
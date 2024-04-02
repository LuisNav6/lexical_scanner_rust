
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
    CIN,
    COUT,

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
    
    //Incrementador
    INCREMENT,
    
    //Decrementador
    DECREMENT,

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
        "cin" => TokenType::CIN,
        "cout" => TokenType::COUT,
        _ => TokenType::ID,
    }
}

// Función para realizar el análisis léxico y devolver los tokens
fn get_token(content: &str) -> (Vec<(TokenType, String, usize, usize)>, Vec<(TokenType, String, usize, usize)>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut lineno = 1;
    let mut state = StateType::Start;
    let mut token_string = String::new();
    let mut linepos = 0;
    let bufsize = content.len();
    let mut column_number = 0;
    while linepos <= bufsize {
        let c = get_next_char(content, &mut linepos, bufsize);
        match state {
            StateType::Start => {
                if c == '\n' {
                    lineno += 1;
                    column_number = 1;
                }
                if c.is_whitespace() {
                    // Ignorar espacios en blanco
                    column_number +=1;
                } else if c.is_ascii_alphabetic() || c == '_' {
                    state = StateType::InId;
                    token_string.push(c);
                    column_number +=1;
                } else if c.is_digit(10) {
                    state = StateType::InNum;
                    token_string.push(c);
                    column_number +=1;
                } else if c == '/' {
                    let next_char = get_next_char(content, &mut linepos, bufsize);
                    if next_char == '/' {
                        let next_char = get_next_char(content, &mut linepos, bufsize);
                        if next_char == '\n' {
                            lineno += 1;
                        } else {
                            unget_next_char(&mut linepos);
                            state = StateType::InComment;
                            lineno += 1;
                        }
                    } else if next_char == '*' {
                        lineno += 1;
                        let next_char = get_next_char(content, &mut linepos, bufsize);
                        if next_char == '\n' {
                            lineno += 1;
                        } else {
                            unget_next_char(&mut linepos);
                            state = StateType::InMultiComment;
                            lineno += 1;
                        }
                    } else {
                        tokens.push((TokenType::DIVIDE, "/".to_string(), lineno, column_number - 1));
                        unget_next_char(&mut linepos)
                    }
                } else {
                    match c {
                        '=' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::EQ, "==".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::ASSIGN, "=".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '!' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::NEQ, "!=".to_string(), lineno, column_number - 1));
                            } else {
                                errors.push((TokenType::ERROR, "!".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '<' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::LTE, "<=".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::LT, "<".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '>' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::GTE, ">=".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::GT, ">".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '+' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '+' {
                                tokens.push((TokenType::INCREMENT, "++".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::PLUS, "+".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '-' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '-' {
                                tokens.push((TokenType::DECREMENT, "--".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::MINUS, "-".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
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
                        _ => errors.push((TokenType::ERROR, c.to_string(), lineno, column_number - 1)),
                    }
                }
            }
            StateType::InId => {
                if c.is_ascii_alphanumeric() || c == '_' {
                    token_string.push(c);
                } else {
                    tokens.push((reserved_lookup(&token_string), token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
             StateType::InNum => {
        if c.is_digit(10) {
            token_string.push(c);
        } else if c == '.' {
            state = StateType::InReal;
            token_string.push(c);
        } else {
            tokens.push((TokenType::NumInt, token_string.clone(), lineno, (column_number - 1)));
            token_string.clear();
            state = StateType::Start;
            unget_next_char(&mut linepos); // Retornar un carácter
        }
    }
    StateType::InReal => {
        if c.is_digit(10) {
            token_string.push(c);
        } else if token_string.ends_with('.') {
            errors.push((TokenType::ERROR, token_string.clone(), lineno, (column_number - 1)));
            token_string.clear();
            state = StateType::Start;
            unget_next_char(&mut linepos);
        } else {
            tokens.push((TokenType::NumReal, token_string.clone(), lineno, (column_number - 1)));
            token_string.clear();
            state = StateType::Start;
            unget_next_char(&mut linepos); // Retornar un caráct
        }
    }
            StateType::InComment => {
                if c == '\n' || c == '\0' {
                    state = StateType::Start;
                    column_number = 1;
                }
            }
            StateType::InMultiComment => {
                if c == '*' {
                    lineno += 1;
                    let next_char = get_next_char(content, &mut linepos, bufsize);
                    if next_char == '/' {
                        state = StateType::Start;
                        lineno += 1;
                    } else {
                        unget_next_char(&mut linepos)
                    }
                } else if c == '\0' {
                    tokens.push((TokenType::InMultipleComment, "/*".to_string(), lineno, column_number - 1));
                    println!("Error: '/*' Multiline comment not closed.");
                    state = StateType::EndFile;
                }
            }
            StateType::EndFile => {
                tokens.push((TokenType::ENDFILE, "\0".to_string(), lineno, column_number - 1));
                break; // Salir del bucle while
            }
            _ => (),
        }
    }
    (tokens, errors)
}


fn main() {
    let file_content = r#" main sum@r 3.14+main)if{32.algo
34.34.34.34
{
int x,y,z;
real a,b,c;
 suma=45;
x=32.32;
x=23;
y=2+3-1;
z=y+7;
y=y+1;
a=24.0+4-1/3*2+34-1;
x=(5-3)*(8/2);
y=5+3-2*4/7-9;
z=8/2+15*4;
y=14.54;
if(2>3)then
        y=a+3;
  else
      if(4>2 && )then
             b=3.2;
       else
           b=5.0;
       end;
       y=y+1;
end;
a++;
c--;
x=3+4;
do
   y=(y+1)*2+1;
   while(x>7){x=6+8/9*8/3;   
    cin x; 
   mas=36/7; 
   };

 until(y=


=



5);
 while(y==0){
    cin mas;
    cout x;
};
}
    "#;
    
    let (tokens, errors) = get_token(&file_content);
    
    println!("Tokens:");
    for (token_type, lexeme, line, column) in &tokens {
        println!(
            "Type: {:?}, Lexeme: {}, Line: {}, Column: {}",
            token_type, lexeme, line, column
        );
    }

    println!("\nErrors:");
    for (error_type, lexeme, line, column) in &errors {
        println!(
            "Type: {:?}, Lexeme: {}, Line: {}, Column: {}",
            error_type, lexeme, line, column
        );
    }
}

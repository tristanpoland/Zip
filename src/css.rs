pub mod css {
    use cssparser::{Parser, ParserInput, Token};

    #[derive(Debug, Clone)]
    pub struct StyleSheet {
        pub rules: Vec<Rule>,
    }

    #[derive(Debug, Clone)]
    pub struct Rule {
        pub selectors: Vec<Selector>,
        pub declarations: Vec<Declaration>,
    }

    #[derive(Debug, Clone)]
    pub struct Selector {
        pub tag_name: Option<String>,
        pub id: Option<String>,
        pub classes: Vec<String>,
    }

    #[derive(Debug, Clone)]
    pub struct Declaration {
        pub name: String,
        pub value: String,
    }

    pub struct CSSParser;

    impl CSSParser {
        pub fn parse(source: &str) -> Result<StyleSheet, crate::BrowserError> {
            let mut input = ParserInput::new(source);
            let mut parser = Parser::new(&mut input);
            let mut stylesheet = StyleSheet { rules: Vec::new() };
            
            while !parser.is_exhausted() {
                parser.skip_whitespace();
                
                // Collect multiple selectors
                let mut selectors = Vec::new();
                loop {
                    let selector = match Self::parse_selector(&mut parser) {
                        Ok(s) => s,
                        Err(_) => return Err(crate::BrowserError::CSSError("Failed to parse selector".into())),
                    };
                    selectors.push(selector);
                    
                    // Check for comma to continue parsing selectors
                    if parser.try_parse(|p| p.expect_comma()).is_err() {
                        break;
                    }
                }
                
                let declarations = match Self::parse_declarations(&mut parser) {
                    Ok(d) => d,
                    Err(_) => return Err(crate::BrowserError::CSSError("Failed to parse declarations".into())),
                };
                
                stylesheet.rules.push(Rule {
                    selectors,
                    declarations,
                });
            }
            
            Ok(stylesheet)
        }

        fn parse_selector<'i>(parser: &mut Parser<'i, '_>) -> Result<Selector, cssparser::ParseError<'i, ()>> {
            let mut selector = Selector {
                tag_name: None,
                id: None,
                classes: Vec::new(),
            };
        
            loop {
                let token = match parser.next_including_whitespace() {
                    Ok(t) => t,
                    Err(_) => break,
                };
        
                match token {
                    Token::Ident(name) => {
                        selector.tag_name = Some(name.to_string());
                    }
                    Token::IDHash(id) => {
                        selector.id = Some(id.to_string());
                    }
                    Token::Delim('.') => {
                        // Store the next token result before matching on it
                        let next = parser.next();
                        if let Ok(Token::Ident(class)) = next {
                            selector.classes.push(class.to_string());
                        }
                    }
                    Token::CurlyBracketBlock => break,
                    _ => {}
                }
            }
            
            Ok(selector)
        }

        fn parse_declarations<'i>(parser: &mut Parser<'i, '_>) -> Result<Vec<Declaration>, cssparser::ParseError<'i, ()>> {
            let mut declarations = Vec::new();
            let mut current_name: Option<String> = None;
            let mut current_value = String::new();
            
            loop {
                let next_token = match parser.next_including_whitespace() {
                    Ok(token) => token,
                    Err(_) => break,
                };

                match (next_token, &current_name) {
                    // Start of a new declaration
                    (Token::Ident(name), None) => {
                        if parser.try_parse(|p| p.expect_colon()).is_ok() {
                            current_name = Some(name.to_string());
                            current_value.clear();
                        } else {
                            break;
                        }
                    }

                    // End of current declaration
                    (Token::Semicolon, Some(_)) => {
                        if let Some(name) = current_name.take() {
                            declarations.push(Declaration {
                                name,
                                value: current_value.trim().to_string(),
                            });
                            current_value.clear();
                        }
                    }

                    // Building the value
                    (token, Some(_)) => {
                        match token {
                            Token::Ident(s) => current_value.push_str(&s),
                            Token::Number { value, .. } => current_value.push_str(&value.to_string()),
                            Token::Dimension { value, unit, .. } => {
                                current_value.push_str(&value.to_string());
                                current_value.push_str(&unit);
                            }
                            Token::Hash(s) => {
                                current_value.push('#');
                                current_value.push_str(&s);
                            }
                            Token::WhiteSpace(_) => current_value.push(' '),
                            Token::CurlyBracketBlock => break,
                            _ => {}
                        }
                    }

                    // End of declarations or unexpected token
                    (Token::CurlyBracketBlock, _) => break,
                    _ => {}
                }
            }
            
            // Handle any remaining declaration
            if let Some(name) = current_name {
                declarations.push(Declaration {
                    name,
                    value: current_value.trim().to_string(),
                });
            }
            
            Ok(declarations)
        }
    }
}
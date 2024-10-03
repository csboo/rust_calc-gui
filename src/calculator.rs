use std::collections::{HashMap, VecDeque};

pub struct Calculator {
    precedence: HashMap<char, (i32, bool)>,
}

impl Calculator {
    pub fn new() -> Self {
        let mut precedence = HashMap::new();
        precedence.insert('+', (1, false)); // left-associative
        precedence.insert('-', (1, false));
        precedence.insert('*', (2, false));
        precedence.insert('/', (2, false));
        precedence.insert('^', (3, true)); // right-associative
        Self { precedence }
    }

    fn tokenize(&self, expr: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut num = String::new();
        let chars: Vec<char> = expr.chars().collect();

        for i in 0..chars.len() {
            let c = chars[i];
            if c.is_digit(10) || c == '.' {
                num.push(c);
            } else {
                if !num.is_empty() {
                    tokens.push(num.clone());
                    num.clear();
                }
                if c.is_whitespace() {
                    continue;
                }
                if c == '-'
                    && (i == 0
                        || chars[i - 1] == '('
                        || self.precedence.contains_key(&chars[i - 1]))
                {
                    num.push(c); // It's a negative number
                } else {
                    tokens.push(c.to_string());
                }
            }
        }
        if !num.is_empty() {
            tokens.push(num);
        }
        tokens
    }

    fn infix_to_postfix(&self, tokens: Vec<String>) -> VecDeque<String> {
        let mut output = VecDeque::new();
        let mut ops = Vec::new();

        for token in tokens {
            if token.chars().all(|c| c.is_digit(10) || c == '.')
                || (token.len() > 1 && token.starts_with('-'))
            {
                output.push_back(token);
            } else if let Some(&(prec, right_assoc)) =
                token.chars().next().and_then(|c| self.precedence.get(&c))
            {
                while let Some(&op) = ops.last() {
                    if let Some(&(op_prec, _)) = self.precedence.get(&op) {
                        if (right_assoc && prec < op_prec) || (!right_assoc && prec <= op_prec) {
                            output.push_back(ops.pop().unwrap().to_string());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                ops.push(token.chars().next().unwrap());
            } else if token == "(" {
                ops.push('(');
            } else if token == ")" {
                while let Some(op) = ops.pop() {
                    if op == '(' {
                        break;
                    }
                    output.push_back(op.to_string());
                }
            }
        }

        while let Some(op) = ops.pop() {
            output.push_back(op.to_string());
        }

        output
    }

    fn evaluate_postfix(&self, postfix: VecDeque<String>) -> f64 {
        let mut stack = Vec::new();

        for token in postfix {
            if token.chars().all(|c| c.is_digit(10) || c == '.')
                || (token.len() > 1 && token.starts_with('-'))
            {
                stack.push(token.parse::<f64>().unwrap());
            } else {
                let val2 = stack.pop().unwrap();
                let val1 = stack.pop().unwrap();
                let result = match token.chars().next().unwrap() {
                    '+' => val1 + val2,
                    '-' => val1 - val2,
                    '*' => val1 * val2,
                    '/' => val1 / val2,
                    '^' => val1.powf(val2),
                    _ => panic!("Unknown operator"),
                };
                stack.push(result);
            }
        }

        stack.pop().unwrap()
    }

    pub fn calculate(&self, expr: &str) -> f64 {
        let tokens = self.tokenize(expr);
        let postfix = self.infix_to_postfix(tokens);
        self.evaluate_postfix(postfix)
    }
}

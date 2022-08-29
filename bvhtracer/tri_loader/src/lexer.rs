use std::str;


#[derive(Clone)]
pub struct Lexer<'a> {
    current_line_number: usize,
    stream_position: usize,
    stream: &'a [u8],
}

#[inline]
fn is_whitespace(ch: u8) -> bool {
    ch == b' ' || ch == b'\\' || ch == b'\t'
}

#[inline]
fn is_newline(ch: u8) -> bool {
    ch == b'\n' || ch == b'\r'
}

#[inline]
fn is_whitespace_or_newline(ch: u8) -> bool {
    is_whitespace(ch) || is_newline(ch)
}

impl<'a> Lexer<'a> {
    pub fn new(stream: &'a str) -> Lexer<'a> {
        Lexer {
            current_line_number: 1,
            stream_position: 0,
            stream: stream.as_bytes(),
        }
    }

    #[inline]
    fn peek(&mut self) -> Option<&u8> {
        self.stream.get(self.stream_position)
    }

    fn advance(&mut self) {
        match self.peek() {
            Some(&ch) if is_newline(ch) => {
                self.current_line_number += 1;
            },
            _ => {}
        }
        self.stream_position += 1;
    }

    fn skip_while<P: Fn(u8) -> bool>(&mut self, predicate: P) -> usize {
        let mut skipped = 0;
        loop {
            match self.peek() {
                Some(&ch) if predicate(ch) => {
                    self.advance();
                    skipped += 1;
                }
                Some(_) | None => {
                    break;
                }
            }
        }

        skipped
    }

    fn skip_unless<P: Fn(u8) -> bool>(&mut self, not_predicate: P) -> usize {
        self.skip_while(|ch| !not_predicate(ch))
    }

    fn skip_comment(&mut self) -> usize {
        match self.peek() {
            Some(b'#') => self.skip_unless(is_newline),
            _ => 0,
        }
    }

    fn skip_whitespace(&mut self) -> usize {
        self.skip_while(is_whitespace)
    }

    fn next_token(&mut self) -> Option<&'a [u8]> {
        self.skip_whitespace();
        self.skip_comment();

        let start_position = self.stream_position;

        match self.peek() {
            Some(&ch) if is_newline(ch) => {
                self.advance();
                self.stream.get(start_position..self.stream_position)
            }
            Some(_) => {
                let skipped = self.skip_unless(|ch| { is_whitespace_or_newline(ch) });
                if skipped > 0 {
                    self.stream.get(start_position..self.stream_position)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


pub struct PeekableLexer<'a> {
    inner: Lexer<'a>,
    cache: Option<Option<&'a str>>,
}

impl<'a> PeekableLexer<'a> {
    pub fn new(lexer: Lexer<'a>) -> PeekableLexer<'a> {
        PeekableLexer {
            inner: lexer,
            cache: None,
        }
    }

    pub fn next_token(&mut self) -> Option<&'a str> {
        match self.cache.take() {
            Some(token) => token,
            None => {
                self.inner.next_token().map(
                    |t| { unsafe { str::from_utf8_unchecked(t) } 
                })
            }
        }
    }

    pub fn peek(&mut self) -> Option<&'a str> {
        match self.cache {
            Some(token) => token,
            None => {
                let next_token = self.inner.next_token().map(
                    |t| { unsafe { str::from_utf8_unchecked(t) } 
                });
                self.cache.replace(next_token);
                next_token
            }
        }
    }
}

impl<'a> Iterator for PeekableLexer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Lexer, 
        PeekableLexer,
    };


    #[test]
    fn test_lexer_empty() {
        let data = String::from(r"");
        let expected: Vec<String> = vec![];
        let lexer = PeekableLexer::new(Lexer::new(&data));
        let result = lexer.map(|token| token.into()).collect::<Vec<String>>();
        
        assert_eq!(result, expected);
    }


    #[test]
    fn test_lexer_one_line() {
        let data = String::from(r"
            -1.920570 0.000680 0.030130 -1.927750 0.102380 0.000170 -1.965040 0.013640 0.009820
        ");
        let expected: Vec<String> = vec![
            "\n", 
            "-1.920570", "0.000680", "0.030130", 
            "-1.927750", "0.102380", "0.000170", 
            "-1.965040", "0.013640", "0.009820",
            "\n"
        ].iter().map(|&str| String::from(str)).collect();
        let lexer = PeekableLexer::new(Lexer::new(&data));
        let result = lexer.map(|token| token.into()).collect::<Vec<String>>();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lexer_multiple_lines() {
        let data = String::from(r"
            -1.920570 0.000680 0.030130 -1.927750 0.102380 0.000170 -1.965040 0.013640 0.009820     \
            -1.879360 0.093640 0.019450 -1.927750 0.102380 0.000170 -1.920570 0.000680 0.030130     \
            -1.879360 0.093640 0.019450 -1.889650 0.172390 -0.028960 -1.927750 0.102380 0.000170    \
            -1.838710 0.162240 -0.008080 -1.889650 0.172390 -0.028960 -1.879360 0.093640 0.019450   \
            -1.838710 0.162240 -0.008080 -1.844990 0.231520 -0.074300 -1.889650 0.172390 -0.028960  \
                                                                                                    \
                                                                                                    \
        ");
        let expected: Vec<String> = vec![
            "\n", 
            "-1.920570", "0.000680", "0.030130", 
            "-1.927750", "0.102380", "0.000170", 
            "-1.965040", "0.013640", "0.009820",
            "\n",
            "-1.879360", "0.093640", "0.019450", 
            "-1.927750", "0.102380", "0.000170", 
            "-1.920570", "0.000680", "0.030130",
            "\n",
            "-1.879360", "0.093640", "0.019450", 
            "-1.889650", "0.172390", "-0.028960", 
            "-1.927750", "0.102380", "0.000170", 
            "\n",
            "-1.838710", "0.162240", "-0.008080", 
            "-1.889650", "0.172390", "-0.028960", 
            "-1.879360", "0.093640", "0.019450", 
            "\n",
            "-1.838710", "0.162240", "-0.008080", 
            "-1.844990", "0.231520", "-0.074300", 
            "-1.889650", "0.172390", "-0.028960", 
            "\n",
            "\n",
            "\n",
        ].iter().map(|&str| String::from(str)).collect();
        let lexer = PeekableLexer::new(Lexer::new(&data));
        let result = lexer.map(|token| token.into()).collect::<Vec<String>>();
        
        assert_eq!(result, expected);
    }
}


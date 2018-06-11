#[derive(Debug)]
pub struct LexError {
    span: (usize, usize),
    code: LexErrorCode,     
}

#[derive(Debug)]
pub enum LexErrorCode {
    
}

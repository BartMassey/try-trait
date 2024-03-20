/*
macro_rules! try_option {
    ($x:expr) => {
        match $x {
            Some(y) => y,
            None => return None,
        }
    }
}
*/

fn collatz_step(x: u8) -> Option<String> {
    Some(x.checked_mul(2)?.checked_add(1)?.to_string())
}

use thiserror::Error;

#[derive(Debug, Error)]
enum CollatzifyError {
    #[error("out-of-domain value")]
    DomainError,
    #[error("collatz overflow")]
    CollatzOverflow,
    #[error("parse error")]
    ParseError,
}
use CollatzifyError::*;

fn collatzify(x: u16) -> Result<u8, CollatzifyError> {
    let x = x.try_into().map_err(|_| DomainError)?;
    let s = collatz_step(x).ok_or(CollatzOverflow)?;
    s.parse().map_err(|_| ParseError)
}

fn main() {
    println!("{}", collatz_step(127).unwrap());
    println!("{}", collatzify(127).unwrap());
}

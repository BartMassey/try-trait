use std::convert::TryFrom;
use thiserror::Error;

fn collatz_step(x: u8) -> Option<String> {
    Some(x.checked_mul(3)?.checked_add(1)?.to_string())
}

#[derive(Debug, Error)]
enum CollatzifyError {
    #[error("out-of-domain value")]
    DomainError(#[source] <u8 as TryFrom<u16>>::Error),
    #[error("collatz overflow")]
    CollatzOverflow,
}
use CollatzifyError::*;

fn collatzify(x: u16) -> Result<u8, CollatzifyError> {
    let x = x.try_into().map_err(DomainError)?;
    let s = collatz_step(x).ok_or(CollatzOverflow)?;
    Ok(s.parse().unwrap())
}

fn main() {
    println!("{}", collatz_step(84).unwrap());
    println!("{}", collatzify(84).unwrap());
    println!("{:?}", collatzify(85));
    println!("{:?}", collatzify(65_534));
}

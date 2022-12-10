pub mod client;
pub mod err;

pub type Result<T> = std::result::Result<T, err::Error>;

#[cfg(test)]
pub mod test {
    #[test]
    fn test_client() {}
}

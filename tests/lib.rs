#[cfg(test)]
mod tests {
    //Test cases are good in rust
    extern crate OddsEnds;
    #[test]
    #[should_panic]
    #[ignore]
    fn english_greeting_correct()
    {
        assert_eq!("hellods", OddsEnds::greetings::english::hello());
    }
}
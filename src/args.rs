pub fn parse_args() -> String {
    std::env::args().nth(1).unwrap()
}

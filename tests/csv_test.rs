#[cfg(test)]
mod tests {
    use golden::get_bar_from_csv;
    #[test]
    fn get_bar_from_csv_test() {
        assert_eq!(4, get_bar_from_csv("SPY"));
    }
}

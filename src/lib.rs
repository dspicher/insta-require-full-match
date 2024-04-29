#[cfg(test)]
mod tests {
    #[test]
    fn test_insta() {
        insta::assert_json_snapshot!(1+1);
    }
}

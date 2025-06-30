pub fn transform(input: &str) -> String {
    input.chars().rev().collect::<String>().to_uppercase()
}
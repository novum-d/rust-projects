pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let full_name = "naruse masanobu";
        let tokens: Vec<&str> = full_name.split(' ').collect();
        let last_name = tokens[0];
        println!("{}", last_name); // naruse
        assert_eq!(result, 4);
    }
}

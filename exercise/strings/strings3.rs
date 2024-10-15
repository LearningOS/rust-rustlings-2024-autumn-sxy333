// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    /*
    // TODO: Remove whitespace from both ends of a string!
    let ret : String;
    let mut start : usize = 0;
    let mut end : usize = 0;
    let mut idx : usize = 0;
    for i in input.chars() {
        if i != ' ' {
            start = idx;
            break;
        }
        idx = idx + 1;
    }

    idx = 0;
    for i in input.chars() {
        if i != ' ' {
            end = idx;
        }
        idx = idx + 1;
    }
    ret = input[start .. end + 1].to_string();
    ret
    */
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let ret = input.to_owned() + " world!";
    ret
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let ret = input.replace("cars", "balloons");
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}

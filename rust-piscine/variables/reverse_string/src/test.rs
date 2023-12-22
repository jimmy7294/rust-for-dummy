#[cfg(test)]

mod tests {
    use crate::reverse_string::*;
    #[test]
    fn test_rev_string() {
        assert_eq!(rev_str("hello"), "olleh");
        assert_eq!(rev_str("hello world"), "dlrow olleh");
        assert_eq!(rev_str("123456789"), "987654321");
    }
}

/*
In Rust, the crate keyword refers to the current crate (a binary or library). When you use crate::, you're specifying an absolute path that starts from the root of your crate.
In your main.rs file, you're able to use "mod reverse_string" and "use reverse_string::*" because main.rs is at the root of your crate. This means that main.rs and reverse_string.rs are at the same level in the module hierarchy.
However, in your test module, you're in a different context. The test module is nested inside your main module, so it doesn't have direct access to the reverse_string module. To access reverse_string from your test module,
you need to use crate:: to go to the root of your crate and then navigate to reverse_string.
So, "use crate::reverse_string::*" in your test module is equivalent to "use reverse_string::*" in your main.rs file. Both lines are importing all items from the reverse_string module, but they're doing so from different locations in your crate's module hierarchy.
*/

/// prints line(s) like `print!()`, but replaces every `\n` with `\r\n`
/// Supports formatting strings. See `println` documentation for how to use this.
#[macro_export]
macro_rules! printr {
    ($($arg:tt)*) => {
        print!($($arg)*).replace("\n", "\r\n");
    };
}

/// prints line(s) like `println!()`, but replaces every `\n` with `\r\n`,
/// with another `\r\n` appended to the end.
/// Supports formatting strings. See `println` documentation for how to use this.
#[macro_export]
macro_rules! printrn {
    () => {
        print!("\r\n")
    };
    ($($arg:tt)*) => {
        let start = format!($($arg)*).replace("\n", "\r\n");
        print!("{}\r\n", start)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_newline() {
        printrn!();
    }

    #[test]
    fn print_text_alone() {
        printrn!("If you can see this message, print_text_alone test worked.");
    }

    #[test]
    fn print_text_with_formatting() {
        let pi: f64 = 3.14;
        let pi_name: &str = "pi";
        printrn!("Approximate value of {}: {}", pi_name, pi);
        printrn!(
            "{}, {} worked.",
            "If you can see this message",
            "print_text_with_formatting"
        );
    }
}

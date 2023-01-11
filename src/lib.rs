/// formats a string like `format!()`, but replaces every `\n` with `\r\n`.
#[macro_export]
macro_rules! formatr {
    ($($arg:tt)*) => {
        format!($($arg)*).replace("\n", "\r\n")
    };
}

/// formats a string like `format!()`, but replaces every `\n` with `\r\n`, and appends `\r\n` to the end.
#[macro_export]
macro_rules! formatrn {
    ($($arg:tt)*) => {
        $crate::formatr!($($arg)*) + "\r\n"
    };
}

/// prints line(s) like `print!()`, but replaces every `\n` with `\r\n`
/// Supports formatting strings. See `println` documentation for how to use this.
#[macro_export]
macro_rules! printr {
    ($($arg:tt)*) => {
        print!("{}", $crate::formatr!($($arg)*))
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
        print!("{}", $crate::formatrn!($($arg)*))
    };
}

#[cfg(test)]
mod tests {

    static A: f32 = 6.;
    static B: f32 = 8.;
    static C: f32 = 10.;
    static STRING_EXAMPLE: &str = "henlo";
    use super::*;

    #[test]
    fn formatr_test() {
        assert_eq!(formatr!("test\n"), "test\r\n");
        assert_eq!(formatr!("n{}\n", 5), "n5\r\n");
    }

    #[test]
    fn formatrn_test() {
        assert_eq!(formatrn!("test"), "test\r\n");
        assert_eq!(formatrn!("n{}", 5), "n5\r\n");
    }

    #[test]
    fn printr_test() {
        printr!("this is A test of printr");
        printr!("{}^2 + {}^2 = {}^2", A, B, C);
        printr!("{}", STRING_EXAMPLE);
    }

    #[test]
    fn printrn_test() {
        printrn!();
        printrn!("this is A test of printrn");
        printrn!("{}^2 + {}^2 = {}^2", A, B, C);
        printrn!("{}", STRING_EXAMPLE);
    }
}

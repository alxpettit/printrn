#[macro_export]
macro_rules! formatr {
    ($($arg:tt)*) => {
        format!($($arg)*).replace("\n", "\r\n")
    };
}

/// prints line(s) like `print!()`, but replaces every `\n` with `\r\n`
/// Supports formatting strings. See `println` documentation for how to use this.
#[macro_export]
macro_rules! printr {
    ($($arg:tt)*) => {
        let start = formatr!($($arg)*);
        print!("{}", start);
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
        let start = formatr!($($arg)*);
        print!("{}\r\n", start)
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
    fn printr() {
        printr!("this is A test of printr");
        printr!("{}^2 + {}^2 = {}^2", A, B, C);
        printr!("{}", STRING_EXAMPLE);
    }

    #[test]
    fn printrn() {
        printrn!("this is A test of printrn");
        printrn!("{}^2 + {}^2 = {}^2", A, B, C);
        printrn!("{}", STRING_EXAMPLE);
    }
}

use serial_test::serial;

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
        let start = format!($($arg)*).replace("\n", "\r\n");
        print!("{}\r\n", start)
    };
}

/// TODO: maybe figure out how to test the output?
/// the only way I can think to do this involves calling the tests and getting output as child process.
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::read_to_string;
    use std::io::Read;
    use std::process::{Command, Stdio};
    use test_toolbox::{capture, expect};

    #[test]
    #[serial]
    fn print_newline() {
        let args: Vec<String> = env::args().skip(1).collect();
        //write!();
        if !args.contains(&"running-self".to_string()) {
            let mut run_self = Command::new("cargo")
                .arg("test")
                .arg("print_newline")
                .arg("--")
                .arg("running-self")
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap();
            let stdout = String::from_utf8_lossy(&run_self.stdout);
            println!("{}", stdout);
            assert!(stdout.contains("\r\n"));
        }

        printrn!("test");
    }

    #[test]
    #[serial]
    fn print_text_alone() {
        // Expected output:
        // If you can see this message, print_text_alone test #1 (printr) worked.
        // If you can see this message, print_text_alone test #2 (printrn) worked.
        {
            let out = capture! {{
                //printr!("If you can see this message, print_text_alone test #1 (printr) worked.\n")
                printrn!("If you can see this message, print_text_alone test #2 (printrn) worked.");
            }};

            //let mut out: Vec<u8> = Vec::new();
            // my_gag.read_to_end(&mut out).unwrap();
            //   println!("<<{:#?}>>", );
        }
    }

    #[test]
    #[serial]
    fn print_text_with_formatting() {
        // Expected output:
        // If you can see this message,
        //  print_text_with_formatting test #1 (printr) worked.
        // If you can see this message,
        //  print_text_with_formatting test #2 (printrn) worked.
        let pi: f64 = 3.14;
        let pi_name: &str = "pi";
        printrn!("Approximate value of {}: {}", pi_name, pi);
        printr!(
            "{},\n {} test #1 (printr) worked.\n",
            "If you can see this message",
            "print_text_with_formatting"
        );
        printrn!(
            "{},\n {} test #2 (printrn) worked.",
            "If you can see this message",
            "print_text_with_formatting"
        );
    }
}

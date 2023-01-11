use printrn::*;

fn main() {
    let fmt_str = formatrn!("This string\nis formatted {}", "UwU");
    println!("{}", fmt_str);

    printr!("This string is printed with no automatic trailing newline.");
}

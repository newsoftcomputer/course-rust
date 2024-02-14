
use crossterm::style::Stylize;
use crossterm::style::Attribute;


fn main() {

    println!("{}", "Bold".bold());
    println!("{}", "Underlined".underlined());
    println!("{}", "Negative".negative());

    println!(
        "{} Underlined {} No Underline",
        Attribute::Underlined,
        Attribute::NoUnderline
    );

}

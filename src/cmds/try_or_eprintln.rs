use std::error::Error;

pub trait TryOrEPrintln {
    fn try_or_eprintln(self);
}

impl TryOrEPrintln for Result<(), Box<dyn Error>> {
    fn try_or_eprintln(self) {
        match self {
            Ok(t) => t,
            Err(e) => println!("{}", e),
        };
    }
}

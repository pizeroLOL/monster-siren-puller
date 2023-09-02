use std::error::Error;

pub trait OkOrEPrintln {
    fn ok_or_eprintln(self);
}

impl OkOrEPrintln for Result<(), Box<dyn Error>> {
    fn ok_or_eprintln(self) {
        match self {
            Ok(t) => t,
            Err(e) => println!("{}", e),
        };
    }
}

use std::{fmt::Display, str::FromStr};

pub const SPECIAL_START_CHARS: [char; 1] = ['_'];

#[derive(Debug)]
pub enum Keyword {
    // Foo,
    // Bar,
    // Baz,
    // Hello,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match self {
            // Self::Foo => write!(f, "foo"),
            // Self::Bar => write!(f, "bar"),
            // Self::Baz => write!(f, "baz"),
            // Self::Hello => write!(f, "_hello"),
        // }
        Ok(())
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // match s {
        //     "foo" => Ok(Self::Foo),
        //     "bar" => Ok(Self::Bar),
        //     "baz" => Ok(Self::Baz),
        //     "_hello" => Ok(Self::Hello),
        //     _ => Err(()),
        // }
        Err(())
    }
}

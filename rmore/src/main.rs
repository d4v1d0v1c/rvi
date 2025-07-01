// Usage: bmore [-acdir] [-lines] [+linenum | +/pattern] name1 name2 ...

use util::{COPYRIGHT, Options};

fn main() {
    let opts = Options::parse().unwrap_or_default();
    println!("copyright: {COPYRIGHT:?}");
    println!("{opts:#?}");
}


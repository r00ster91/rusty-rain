use crate::{ABOUT, AUTHOR, VERSION};
use clap::{App, Arg};

type COLOR = (u8, u8, u8);
pub fn cargs() -> (COLOR, (u32, u32), bool, COLOR) {
    let matches = App::new("Rusty Rain")
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::with_name("color")
                .short("C")
                .long("color")
                .help(
                    "Set color of Rain with color string name or tuple
                white,
                red,
                blue,
                green,
                r,g,b",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("head")
                .short("H")
                .long("head")
                .help(
                    "Set the color of the first char in Rain.
                white,
                red,
                blue,
                green,
                r,g,b",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("characters")
                .short("c")
                .long("chars")
                .help(
                    "Set what kind of characters are printed as rain.
                jap      - for Japanese characters
                bin      - for binary characters
                alphalow - for lowercase characters
                alphaup  - for uppercase characters
                num      - for numbers",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shade")
                .short("s")
                .long("shade")
                .help("Set Rain shading to fade or stay constant")
                .takes_value(false),
        )
        .get_matches();

    let color = match matches.value_of("color").unwrap_or("green") {
        "white" => (255, 255, 255),
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "blue" => (0, 0, 255),
        a => a.to_string().into_tuple(),
    };

    let head = match matches.value_of("head").unwrap_or("white") {
        "white" => (255, 255, 255),
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "blue" => (0, 0, 255),
        a => a.to_string().into_tuple(),
    };

    let characters = match matches.value_of("characters").unwrap_or("bin") {
        "jap" => (65382, 65437),
        "bin" => (48, 50),
        "alphalow" => (97, 122),
        "alphaup" => (65, 90),
        "num" => (48, 57),
        "alphanumsim" => (33, 127),
        _ => (48, 50),
    };

    let shading = matches.is_present("shade");

    (color, characters, shading, head)
}

impl StrTuple for String {
    type Tuple = (u8, u8, u8);
    fn into_tuple(self) -> Self::Tuple {
        let mut nums = Vec::new();
        for num in self.split(',') {
            nums.push(
                num.parse::<u8>()
                    .expect("This is not the correct format, expecting 0,0,0 or name like white"),
            );
        }
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        (a, b, c)
    }
}

trait StrTuple {
    type Tuple;
    fn into_tuple(self) -> Self::Tuple;
}

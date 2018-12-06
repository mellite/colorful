use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightGray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Grey0,
    NavyBlue,
    DarkBlue,
    Blue3a,
    Blue3b,
    Blue1,
    DarkGreen,
    DeepSkyBlue4a,
    DeepSkyBlue4b,
    DeepSkyBlue4c,
    DodgerBlue3,
    DodgerBlue2,
    Green4,
    SpringGreen4,
    Turquoise4,
    DeepSkyBlue3a,
    DeepSkyBlue3b,
    DodgerBlue1,
    Green3a,
    SpringGreen3a,
    DarkCyan,
    LightSeaGreen,
    DeepSkyBlue2,
    DeepSkyBlue1,
    Green3b,
    SpringGreen3b,
    SpringGreen2a,
    Cyan3,
    DarkTurquoise,
    Turquoise2,
    Green1,
    SpringGreen2b,
    SpringGreen1,
    MediumSpringGreen,
    Cyan2,
    Cyan1,
    DarkRed1,
    DeepPink4a,
    Purple4a,
    Purple4b,
    Purple3,
    BlueViolet,
    Orange4a,
    Grey37,
    MediumPurple4,
    SlateBlue3a,
    SlateBlue3b,
    RoyalBlue1,
    Chartreuse4,
    DarkSeaGreen4a,
    PaleTurquoise4,
    SteelBlue,
    SteelBlue3,
    CornflowerBlue,
    Chartreuse3a,
    DarkSeaGreen4b,
    CadetBlue2,
    CadetBlue1,
    SkyBlue3,
    SteelBlue1a,
    Chartreuse3b,
    PaleGreen3a,
    SeaGreen3,
    Aquamarine3,
    MediumTurquoise,
    SteelBlue1b,
    Chartreuse2a,
    SeaGreen2,
    SeaGreen1a,
    SeaGreen1b,
    Aquamarine1a,
    DarkSlateGray2,
    DarkRed2,
    DeepPink4b,
    DarkMagenta1,
    DarkMagenta2,
    DarkViolet1a,
    Purple1a,
    Orange4b,
    LightPink4,
    Plum4,
    MediumPurple3a,
    MediumPurple3b,
    SlateBlue1,
    Yellow4a,
    Wheat4,
    Grey53,
    LightSlateGrey,
    MediumPurple,
    LightSlateBlue,
    Yellow4b,
    DarkOliveGreen3a,
    DarkGreenSea,
    LightSkyBlue3a,
    LightSkyBlue3b,
    SkyBlue2,
    Chartreuse2b,
    DarkOliveGreen3b,
    PaleGreen3b,
    DarkSeaGreen3a,
    DarkSlateGray3,
    SkyBlue1,
    Chartreuse1,
    LightGreen2,
    LightGreen3,
    PaleGreen1a,
    Aquamarine1b,
    DarkSlateGray1,
    Red3a,
    DeepPink4c,
    MediumVioletRed,
    Magenta3a,
    DarkViolet1b,
    Purple1b,
    DarkOrange3a,
    IndianRed1a,
    HotPink3a,
    MediumOrchid3,
    MediumOrchid,
    MediumPurple2a,
    DarkGoldenrod,
    LightSalmon3a,
    RosyBrown,
    Grey63,
    MediumPurple2b,
    MediumPurple1,
    Gold3a,
    DarkKhaki,
    NavajoWhite3,
    Grey69,
    LightSteelBlue3,
    LightSteelBlue,
    Yellow3a,
    DarkOliveGreen3,
    DarkSeaGreen3b,
    DarkSeaGreen2,
    LightCyan3,
    LightSkyBlue1,
    GreenYellow,
    DarkOliveGreen2,
    PaleGreen1b,
    DarkSeaGreen5b,
    DarkSeaGreen5a,
    PaleTurquoise1,
    Red3b,
    DeepPink3a,
    DeepPink3b,
    Magenta3b,
    Magenta3c,
    Magenta2a,
    DarkOrange3b,
    IndianRed1b,
    HotPink3b,
    HotPink2,
    Orchid,
    MediumOrchid1a,
    Orange3,
    LightSalmon3b,
    LightPink3,
    Pink3,
    Plum3,
    Violet,
    Gold3b,
    LightGoldenrod3,
    Tan,
    MistyRose3,
    Thistle3,
    Plum2,
    Yellow3b,
    Khaki3,
    LightGoldenrod2a,
    LightYellow3,
    Grey84,
    LightSteelBlue1,
    Yellow2,
    DarkOliveGreen1a,
    DarkOliveGreen1b,
    DarkSeaGreen1,
    Honeydew2,
    LightCyan1,
    Red1,
    DeepPink2,
    DeepPink1a,
    DeepPink1b,
    Magenta2b,
    Magenta1,
    OrangeRed1,
    IndianRed1c,
    IndianRed1d,
    HotPink1a,
    HotPink1b,
    MediumOrchid1b,
    DarkOrange,
    Salmon1,
    LightCoral,
    PaleVioletRed1,
    Orchid2,
    Orchid1,
    Orange1,
    SandyBrown,
    LightSalmon1,
    LightPink1,
    Pink1,
    Plum1,
    Gold1,
    LightGoldenrod2b,
    LightGoldenrod2c,
    NavajoWhite1,
    MistyRose1,
    Thistle1,
    Yellow1,
    LightGoldenrod1,
    Khaki1,
    Wheat1,
    CornSilk1,
    Grey100,
    Grey3,
    Grey7,
    Grey11,
    Grey15,
    Grey19,
    Grey23,
    Grey27,
    Grey30,
    Grey35,
    Grey39,
    Grey42,
    Grey46,
    Grey50,
    Grey54,
    Grey58,
    Grey62,
    Grey66,
    Grey70,
    Grey74,
    Grey78,
    Grey82,
    Grey85,
    Grey89,
    Grey93,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{}",
                match self {
                    Color::Black => "0",
                    Color::Red => "1",
                    Color::Green => "2",
                    Color::Yellow => "3",
                    Color::Blue => "4",
                    Color::Magenta => "5",
                    Color::Cyan => "6",
                    Color::LightGray => "7",
                    Color::DarkGray => "8",
                    Color::LightRed => "9",
                    Color::LightGreen => "10",
                    Color::LightYellow => "11",
                    Color::LightBlue => "12",
                    Color::LightMagenta => "13",
                    Color::LightCyan => "14",
                    Color::White => "15",
                    Color::Grey0 => "16",
                    Color::NavyBlue => "17",
                    Color::DarkBlue => "18",
                    Color::Blue3a => "19",
                    Color::Blue3b => "20",
                    Color::Blue1 => "21",
                    Color::DarkGreen => "22",
                    Color::DeepSkyBlue4a => "23",
                    Color::DeepSkyBlue4b => "24",
                    Color::DeepSkyBlue4c => "25",
                    Color::DodgerBlue3 => "26",
                    Color::DodgerBlue2 => "27",
                    Color::Green4 => "28",
                    Color::SpringGreen4 => "29",
                    Color::Turquoise4 => "30",
                    Color::DeepSkyBlue3a => "31",
                    Color::DeepSkyBlue3b => "32",
                    Color::DodgerBlue1 => "33",
                    Color::Green3a => "34",
                    Color::SpringGreen3a => "35",
                    Color::DarkCyan => "36",
                    Color::LightSeaGreen => "37",
                    Color::DeepSkyBlue2 => "38",
                    Color::DeepSkyBlue1 => "39",
                    Color::Green3b => "40",
                    Color::SpringGreen3b => "41",
                    Color::SpringGreen2a => "42",
                    Color::Cyan3 => "43",
                    Color::DarkTurquoise => "44",
                    Color::Turquoise2 => "45",
                    Color::Green1 => "46",
                    Color::SpringGreen2b => "47",
                    Color::SpringGreen1 => "48",
                    Color::MediumSpringGreen => "49",
                    Color::Cyan2 => "50",
                    Color::Cyan1 => "51",
                    Color::DarkRed1 => "52",
                    Color::DeepPink4a => "53",
                    Color::Purple4a => "54",
                    Color::Purple4b => "55",
                    Color::Purple3 => "56",
                    Color::BlueViolet => "57",
                    Color::Orange4a => "58",
                    Color::Grey37 => "59",
                    Color::MediumPurple4 => "60",
                    Color::SlateBlue3a => "61",
                    Color::SlateBlue3b => "62",
                    Color::RoyalBlue1 => "63",
                    Color::Chartreuse4 => "64",
                    Color::DarkSeaGreen4a => "65",
                    Color::PaleTurquoise4 => "66",
                    Color::SteelBlue => "67",
                    Color::SteelBlue3 => "68",
                    Color::CornflowerBlue => "69",
                    Color::Chartreuse3a => "70",
                    Color::DarkSeaGreen4b => "71",
                    Color::CadetBlue2 => "72",
                    Color::CadetBlue1 => "73",
                    Color::SkyBlue3 => "74",
                    Color::SteelBlue1a => "75",
                    Color::Chartreuse3b => "76",
                    Color::PaleGreen3a => "77",
                    Color::SeaGreen3 => "78",
                    Color::Aquamarine3 => "79",
                    Color::MediumTurquoise => "80",
                    Color::SteelBlue1b => "81",
                    Color::Chartreuse2a => "82",
                    Color::SeaGreen2 => "83",
                    Color::SeaGreen1a => "84",
                    Color::SeaGreen1b => "85",
                    Color::Aquamarine1a => "86",
                    Color::DarkSlateGray2 => "87",
                    Color::DarkRed2 => "88",
                    Color::DeepPink4b => "89",
                    Color::DarkMagenta1 => "90",
                    Color::DarkMagenta2 => "91",
                    Color::DarkViolet1a => "92",
                    Color::Purple1a => "93",
                    Color::Orange4b => "94",
                    Color::LightPink4 => "95",
                    Color::Plum4 => "96",
                    Color::MediumPurple3a => "97",
                    Color::MediumPurple3b => "98",
                    Color::SlateBlue1 => "99",
                    Color::Yellow4a => "100",
                    Color::Wheat4 => "101",
                    Color::Grey53 => "102",
                    Color::LightSlateGrey => "103",
                    Color::MediumPurple => "104",
                    Color::LightSlateBlue => "105",
                    Color::Yellow4b => "106",
                    Color::DarkOliveGreen3a => "107",
                    Color::DarkGreenSea => "108",
                    Color::LightSkyBlue3a => "109",
                    Color::LightSkyBlue3b => "110",
                    Color::SkyBlue2 => "111",
                    Color::Chartreuse2b => "112",
                    Color::DarkOliveGreen3b => "113",
                    Color::PaleGreen3b => "114",
                    Color::DarkSeaGreen3a => "115",
                    Color::DarkSlateGray3 => "116",
                    Color::SkyBlue1 => "117",
                    Color::Chartreuse1 => "118",
                    Color::LightGreen2 => "119",
                    Color::LightGreen3 => "120",
                    Color::PaleGreen1a => "121",
                    Color::Aquamarine1b => "122",
                    Color::DarkSlateGray1 => "123",
                    Color::Red3a => "124",
                    Color::DeepPink4c => "125",
                    Color::MediumVioletRed => "126",
                    Color::Magenta3a => "127",
                    Color::DarkViolet1b => "128",
                    Color::Purple1b => "129",
                    Color::DarkOrange3a => "130",
                    Color::IndianRed1a => "131",
                    Color::HotPink3a => "132",
                    Color::MediumOrchid3 => "133",
                    Color::MediumOrchid => "134",
                    Color::MediumPurple2a => "135",
                    Color::DarkGoldenrod => "136",
                    Color::LightSalmon3a => "137",
                    Color::RosyBrown => "138",
                    Color::Grey63 => "139",
                    Color::MediumPurple2b => "140",
                    Color::MediumPurple1 => "141",
                    Color::Gold3a => "142",
                    Color::DarkKhaki => "143",
                    Color::NavajoWhite3 => "144",
                    Color::Grey69 => "145",
                    Color::LightSteelBlue3 => "146",
                    Color::LightSteelBlue => "147",
                    Color::Yellow3a => "148",
                    Color::DarkOliveGreen3 => "149",
                    Color::DarkSeaGreen3b => "150",
                    Color::DarkSeaGreen2 => "151",
                    Color::LightCyan3 => "152",
                    Color::LightSkyBlue1 => "153",
                    Color::GreenYellow => "154",
                    Color::DarkOliveGreen2 => "155",
                    Color::PaleGreen1b => "156",
                    Color::DarkSeaGreen5b => "157",
                    Color::DarkSeaGreen5a => "158",
                    Color::PaleTurquoise1 => "159",
                    Color::Red3b => "160",
                    Color::DeepPink3a => "161",
                    Color::DeepPink3b => "162",
                    Color::Magenta3b => "163",
                    Color::Magenta3c => "164",
                    Color::Magenta2a => "165",
                    Color::DarkOrange3b => "166",
                    Color::IndianRed1b => "167",
                    Color::HotPink3b => "168",
                    Color::HotPink2 => "169",
                    Color::Orchid => "170",
                    Color::MediumOrchid1a => "171",
                    Color::Orange3 => "172",
                    Color::LightSalmon3b => "173",
                    Color::LightPink3 => "174",
                    Color::Pink3 => "175",
                    Color::Plum3 => "176",
                    Color::Violet => "177",
                    Color::Gold3b => "178",
                    Color::LightGoldenrod3 => "179",
                    Color::Tan => "180",
                    Color::MistyRose3 => "181",
                    Color::Thistle3 => "182",
                    Color::Plum2 => "183",
                    Color::Yellow3b => "184",
                    Color::Khaki3 => "185",
                    Color::LightGoldenrod2a => "186",
                    Color::LightYellow3 => "187",
                    Color::Grey84 => "188",
                    Color::LightSteelBlue1 => "189",
                    Color::Yellow2 => "190",
                    Color::DarkOliveGreen1a => "191",
                    Color::DarkOliveGreen1b => "192",
                    Color::DarkSeaGreen1 => "193",
                    Color::Honeydew2 => "194",
                    Color::LightCyan1 => "195",
                    Color::Red1 => "196",
                    Color::DeepPink2 => "197",
                    Color::DeepPink1a => "198",
                    Color::DeepPink1b => "199",
                    Color::Magenta2b => "200",
                    Color::Magenta1 => "201",
                    Color::OrangeRed1 => "202",
                    Color::IndianRed1c => "203",
                    Color::IndianRed1d => "204",
                    Color::HotPink1a => "205",
                    Color::HotPink1b => "206",
                    Color::MediumOrchid1b => "207",
                    Color::DarkOrange => "208",
                    Color::Salmon1 => "209",
                    Color::LightCoral => "210",
                    Color::PaleVioletRed1 => "211",
                    Color::Orchid2 => "212",
                    Color::Orchid1 => "213",
                    Color::Orange1 => "214",
                    Color::SandyBrown => "215",
                    Color::LightSalmon1 => "216",
                    Color::LightPink1 => "217",
                    Color::Pink1 => "218",
                    Color::Plum1 => "219",
                    Color::Gold1 => "220",
                    Color::LightGoldenrod2b => "221",
                    Color::LightGoldenrod2c => "222",
                    Color::NavajoWhite1 => "223",
                    Color::MistyRose1 => "224",
                    Color::Thistle1 => "225",
                    Color::Yellow1 => "226",
                    Color::LightGoldenrod1 => "227",
                    Color::Khaki1 => "228",
                    Color::Wheat1 => "229",
                    Color::CornSilk1 => "230",
                    Color::Grey100 => "231",
                    Color::Grey3 => "232",
                    Color::Grey7 => "233",
                    Color::Grey11 => "234",
                    Color::Grey15 => "235",
                    Color::Grey19 => "236",
                    Color::Grey23 => "237",
                    Color::Grey27 => "238",
                    Color::Grey30 => "239",
                    Color::Grey35 => "240",
                    Color::Grey39 => "241",
                    Color::Grey42 => "242",
                    Color::Grey46 => "243",
                    Color::Grey50 => "244",
                    Color::Grey54 => "245",
                    Color::Grey58 => "246",
                    Color::Grey62 => "247",
                    Color::Grey66 => "248",
                    Color::Grey70 => "249",
                    Color::Grey74 => "250",
                    Color::Grey78 => "251",
                    Color::Grey82 => "252",
                    Color::Grey85 => "253",
                    Color::Grey89 => "254",
                    Color::Grey93 => "255",
                }
        )
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGB {
    // range 0 -255
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct HSL {
    // range 0 ≤ H < 1.0, 0 ≤ S ≤ 1.0 and 0 ≤ L ≤ 1.0:
    h: f32,
    s: f32,
    l: f32,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }
    pub fn unpack(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl ToString for RGB {
    fn to_string(&self) -> String {
        format!("{};{};{}", self.r, self.g, self.b)
    }
}

fn _round(value: f32) -> f32 {
    if value < 0.0 {
        0.0
    } else if value >= 1.0 {
        1.0
    } else {
        value
    }
}

impl HSL {
    pub fn new(h: f32, s: f32, l: f32) -> HSL {
        HSL { h: _round(h), s: _round(s), l: _round(l) }
    }
}

pub fn hsl_to_rgb(hsl: HSL) -> RGB {
    let mut red: f32;
    let mut green: f32;
    let mut blue: f32;
    let var_1: f32;
    let var_2: f32;
    if hsl.s == 0.0 {
        let tmp = hsl.l * 255.0;
        red = tmp;
        green = tmp;
        blue = tmp;
    } else {
        if hsl.l < 0.5 {
            var_2 = hsl.l * (1.0 + hsl.s);
        } else {
            var_2 = (hsl.l + hsl.s) - (hsl.s * hsl.l);
        }
        var_1 = 2.0 * hsl.l - var_2;
        red = 255.0 * hue_2_rgb(var_1, var_2, &mut (hsl.h + (1.0 / 3.0)));
        green = 255.0 * hue_2_rgb(var_1, var_2, &mut hsl.h.clone());
        blue = 255.0 * hue_2_rgb(var_1, var_2, &mut (hsl.h - (1.0 / 3.0)));
    }
    RGB { r: red.round() as u8, g: green.round() as u8, b: blue.round() as u8 }
}

fn hue_2_rgb(v1: f32, v2: f32, vh: &mut f32) -> f32 {
    if *vh < 0.0 {
        *vh += 1.0;
    }
    if *vh > 1.0 {
        *vh -= 1.0;
    }
    if 6.0 * *vh < 1.0 {
        return v1 + (v2 - v1) * 6.0 * *vh;
    }
    if 2.0 * *vh < 1.0 {
        return v2;
    }
    if 3.0 * *vh < 2.0 {
        return v1 + (v2 - v1) * (2.0 / 3.0 - *vh) * 6.0;
    }
    v1
}

#[derive(Copy, Clone)]
pub enum ColorMode {
    SIMPLE,
    RGB,
    HSL,
}

#[derive(Clone)]
pub struct Colorado {
    pub mode: ColorMode,
    pub color: String,
}


impl Default for Colorado {
    fn default() -> Colorado {
        Colorado {
            mode: ColorMode::SIMPLE,
            color: String::default(),
        }
    }
}

impl Colorado {
    pub fn new<T: ToString>(color: T) -> Colorado {
        let c = format!("{}", color.to_string());
        Colorado {
            color: format!("{}", color.to_string()),
            mode: if c.contains(";") {
                ColorMode::RGB
            } else {
                ColorMode::SIMPLE
            }
        }
    }
}


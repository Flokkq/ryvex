use super::key::KeyType;

#[derive(PartialEq)]
pub enum EscapeSequence {
    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowLeft,
}

impl EscapeSequence {
    pub fn as_str(&self) -> &str {
        match self {
            EscapeSequence::ArrowUp => "\x1B[A",
            EscapeSequence::ArrowDown => "\x1B[B",
            EscapeSequence::ArrowRight => "\x1B[C",
            EscapeSequence::ArrowLeft => "\x1B[D",
        }
    }
}

#[derive(PartialEq)]
pub enum KeyCode {
    EscapeSequence(EscapeSequence),
    Nul,
    Soh,
    Stx,
    Etx,
    Eot,
    Enq,
    Ack,
    Bell,
    Backspace,
    Tab,
    LineFeed,
    Vt,
    Ff,
    CarriageReturn,
    So,
    Si,
    Dle,
    Dc1,
    Dc2,
    Dc3,
    Dc4,
    Nak,
    Syn,
    Etb,
    Can,
    Em,
    Sub,
    Esc,
    Fs,
    Gs,
    Rs,
    Us,
    Space,
    Exclamation,
    DoubleQuote,
    Hash,
    Dollar,
    Percent,
    Ampersand,
    Quote,
    LeftParenthesis,
    RightParenthesis,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Colon,
    Semicolon,
    LessThan,
    Equal,
    GreaterThan,
    Question,
    At,
    UpperA,
    UpperB,
    UpperC,
    UpperD,
    UpperE,
    UpperF,
    UpperG,
    UpperH,
    UpperI,
    UpperJ,
    UpperK,
    UpperL,
    UpperM,
    UpperN,
    UpperO,
    UpperP,
    UpperQ,
    UpperR,
    UpperS,
    UpperT,
    UpperU,
    UpperV,
    UpperW,
    UpperX,
    UpperY,
    UpperZ,
    LeftBracket,
    Backslash,
    RightBracket,
    Caret,
    Underscore,
    Grave,
    LowerA,
    LowerB,
    LowerC,
    LowerD,
    LowerE,
    LowerF,
    LowerG,
    LowerH,
    LowerI,
    LowerJ,
    LowerK,
    LowerL,
    LowerM,
    LowerN,
    LowerO,
    LowerP,
    LowerQ,
    LowerR,
    LowerS,
    LowerT,
    LowerU,
    LowerV,
    LowerW,
    LowerX,
    LowerY,
    LowerZ,
    LeftBrace,
    Pipe,
    RightBrace,
    Tilde,
    Del,
}

impl KeyCode {
    pub fn to_key_type(&self) -> KeyType {
        let ascii_char = self.as_str();
        KeyType::char_to_key_type(ascii_char)
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<KeyCode> {
        match bytes {
            [b'\x1B', b'[', b'A'] => {
                Some(KeyCode::EscapeSequence(EscapeSequence::ArrowUp))
            }
            [b'\x1B', b'[', b'B'] => {
                Some(KeyCode::EscapeSequence(EscapeSequence::ArrowDown))
            }
            [b'\x1B', b'[', b'C'] => {
                Some(KeyCode::EscapeSequence(EscapeSequence::ArrowRight))
            }
            [b'\x1B', b'[', b'D'] => {
                Some(KeyCode::EscapeSequence(EscapeSequence::ArrowLeft))
            }
            [single_byte] => KeyCode::from_ascii(*single_byte),
            _ => None,
        }
    }

    pub fn from_ascii(value: u8) -> Option<KeyCode> {
        match value {
            0 => Some(KeyCode::Nul),
            1 => Some(KeyCode::Soh),
            2 => Some(KeyCode::Stx),
            3 => Some(KeyCode::Etx),
            4 => Some(KeyCode::Eot),
            5 => Some(KeyCode::Enq),
            6 => Some(KeyCode::Ack),
            7 => Some(KeyCode::Bell),
            8 => Some(KeyCode::Backspace),
            9 => Some(KeyCode::Tab),
            10 => Some(KeyCode::LineFeed),
            11 => Some(KeyCode::Vt),
            12 => Some(KeyCode::Ff),
            13 => Some(KeyCode::CarriageReturn),
            14 => Some(KeyCode::So),
            15 => Some(KeyCode::Si),
            16 => Some(KeyCode::Dle),
            17 => Some(KeyCode::Dc1),
            18 => Some(KeyCode::Dc2),
            19 => Some(KeyCode::Dc3),
            20 => Some(KeyCode::Dc4),
            21 => Some(KeyCode::Nak),
            22 => Some(KeyCode::Syn),
            23 => Some(KeyCode::Etb),
            24 => Some(KeyCode::Can),
            25 => Some(KeyCode::Em),
            26 => Some(KeyCode::Sub),
            27 => Some(KeyCode::Esc),
            28 => Some(KeyCode::Fs),
            29 => Some(KeyCode::Gs),
            30 => Some(KeyCode::Rs),
            31 => Some(KeyCode::Us),
            32 => Some(KeyCode::Space),
            33 => Some(KeyCode::Exclamation),
            34 => Some(KeyCode::DoubleQuote),
            35 => Some(KeyCode::Hash),
            36 => Some(KeyCode::Dollar),
            37 => Some(KeyCode::Percent),
            38 => Some(KeyCode::Ampersand),
            39 => Some(KeyCode::Quote),
            40 => Some(KeyCode::LeftParenthesis),
            41 => Some(KeyCode::RightParenthesis),
            42 => Some(KeyCode::Asterisk),
            43 => Some(KeyCode::Plus),
            44 => Some(KeyCode::Comma),
            45 => Some(KeyCode::Minus),
            46 => Some(KeyCode::Period),
            47 => Some(KeyCode::Slash),
            48 => Some(KeyCode::Zero),
            49 => Some(KeyCode::One),
            50 => Some(KeyCode::Two),
            51 => Some(KeyCode::Three),
            52 => Some(KeyCode::Four),
            53 => Some(KeyCode::Five),
            54 => Some(KeyCode::Six),
            55 => Some(KeyCode::Seven),
            56 => Some(KeyCode::Eight),
            57 => Some(KeyCode::Nine),
            58 => Some(KeyCode::Colon),
            59 => Some(KeyCode::Semicolon),
            60 => Some(KeyCode::LessThan),
            61 => Some(KeyCode::Equal),
            62 => Some(KeyCode::GreaterThan),
            63 => Some(KeyCode::Question),
            64 => Some(KeyCode::At),
            65 => Some(KeyCode::UpperA),
            66 => Some(KeyCode::UpperB),
            67 => Some(KeyCode::UpperC),
            68 => Some(KeyCode::UpperD),
            69 => Some(KeyCode::UpperE),
            70 => Some(KeyCode::UpperF),
            71 => Some(KeyCode::UpperG),
            72 => Some(KeyCode::UpperH),
            73 => Some(KeyCode::UpperI),
            74 => Some(KeyCode::UpperJ),
            75 => Some(KeyCode::UpperK),
            76 => Some(KeyCode::UpperL),
            77 => Some(KeyCode::UpperM),
            78 => Some(KeyCode::UpperN),
            79 => Some(KeyCode::UpperO),
            80 => Some(KeyCode::UpperP),
            81 => Some(KeyCode::UpperQ),
            82 => Some(KeyCode::UpperR),
            83 => Some(KeyCode::UpperS),
            84 => Some(KeyCode::UpperT),
            85 => Some(KeyCode::UpperU),
            86 => Some(KeyCode::UpperV),
            87 => Some(KeyCode::UpperW),
            88 => Some(KeyCode::UpperX),
            89 => Some(KeyCode::UpperY),
            90 => Some(KeyCode::UpperZ),
            91 => Some(KeyCode::LeftBracket),
            92 => Some(KeyCode::Backslash),
            93 => Some(KeyCode::RightBracket),
            94 => Some(KeyCode::Caret),
            95 => Some(KeyCode::Underscore),
            96 => Some(KeyCode::Grave),
            97 => Some(KeyCode::LowerA),
            98 => Some(KeyCode::LowerB),
            99 => Some(KeyCode::LowerC),
            100 => Some(KeyCode::LowerD),
            101 => Some(KeyCode::LowerE),
            102 => Some(KeyCode::LowerF),
            103 => Some(KeyCode::LowerG),
            104 => Some(KeyCode::LowerH),
            105 => Some(KeyCode::LowerI),
            106 => Some(KeyCode::LowerJ),
            107 => Some(KeyCode::LowerK),
            108 => Some(KeyCode::LowerL),
            109 => Some(KeyCode::LowerM),
            110 => Some(KeyCode::LowerN),
            111 => Some(KeyCode::LowerO),
            112 => Some(KeyCode::LowerP),
            113 => Some(KeyCode::LowerQ),
            114 => Some(KeyCode::LowerR),
            115 => Some(KeyCode::LowerS),
            116 => Some(KeyCode::LowerT),
            117 => Some(KeyCode::LowerU),
            118 => Some(KeyCode::LowerV),
            119 => Some(KeyCode::LowerW),
            120 => Some(KeyCode::LowerX),
            121 => Some(KeyCode::LowerY),
            122 => Some(KeyCode::LowerZ),
            123 => Some(KeyCode::LeftBrace),
            124 => Some(KeyCode::Pipe),
            125 => Some(KeyCode::RightBrace),
            126 => Some(KeyCode::Tilde),
            127 => Some(KeyCode::Del),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            KeyCode::EscapeSequence(seq) => seq.as_str(),
            KeyCode::Nul => "\x00",
            KeyCode::Soh => "\x01",
            KeyCode::Stx => "\x02",
            KeyCode::Etx => "\x03",
            KeyCode::Eot => "\x04",
            KeyCode::Enq => "\x05",
            KeyCode::Ack => "\x06",
            KeyCode::Bell => "\x07",
            KeyCode::Backspace => "\x08",
            KeyCode::Tab => "\x09",
            KeyCode::LineFeed => "\x0A",
            KeyCode::Vt => "\x0B",
            KeyCode::Ff => "\x0C",
            KeyCode::CarriageReturn => "\x0D",
            KeyCode::So => "\x0E",
            KeyCode::Si => "\x0F",
            KeyCode::Dle => "\x10",
            KeyCode::Dc1 => "\x11",
            KeyCode::Dc2 => "\x12",
            KeyCode::Dc3 => "\x13",
            KeyCode::Dc4 => "\x14",
            KeyCode::Nak => "\x15",
            KeyCode::Syn => "\x16",
            KeyCode::Etb => "\x17",
            KeyCode::Can => "\x18",
            KeyCode::Em => "\x19",
            KeyCode::Sub => "\x1A",
            KeyCode::Esc => "\x1B",
            KeyCode::Fs => "\x1C",
            KeyCode::Gs => "\x1D",
            KeyCode::Rs => "\x1E",
            KeyCode::Us => "\x1F",
            KeyCode::Space => " ",
            KeyCode::Exclamation => "!",
            KeyCode::DoubleQuote => "\"",
            KeyCode::Hash => "#",
            KeyCode::Dollar => "$",
            KeyCode::Percent => "%",
            KeyCode::Ampersand => "&",
            KeyCode::Quote => "'",
            KeyCode::LeftParenthesis => "(",
            KeyCode::RightParenthesis => ")",
            KeyCode::Asterisk => "*",
            KeyCode::Plus => "+",
            KeyCode::Comma => ",",
            KeyCode::Minus => "-",
            KeyCode::Period => ".",
            KeyCode::Slash => "/",
            KeyCode::Zero => "0",
            KeyCode::One => "1",
            KeyCode::Two => "2",
            KeyCode::Three => "3",
            KeyCode::Four => "4",
            KeyCode::Five => "5",
            KeyCode::Six => "6",
            KeyCode::Seven => "7",
            KeyCode::Eight => "8",
            KeyCode::Nine => "9",
            KeyCode::Colon => ":",
            KeyCode::Semicolon => ";",
            KeyCode::LessThan => "<",
            KeyCode::Equal => "=",
            KeyCode::GreaterThan => ">",
            KeyCode::Question => "?",
            KeyCode::At => "@",
            KeyCode::UpperA => "A",
            KeyCode::UpperB => "B",
            KeyCode::UpperC => "C",
            KeyCode::UpperD => "D",
            KeyCode::UpperE => "E",
            KeyCode::UpperF => "F",
            KeyCode::UpperG => "G",
            KeyCode::UpperH => "H",
            KeyCode::UpperI => "I",
            KeyCode::UpperJ => "J",
            KeyCode::UpperK => "K",
            KeyCode::UpperL => "L",
            KeyCode::UpperM => "M",
            KeyCode::UpperN => "N",
            KeyCode::UpperO => "O",
            KeyCode::UpperP => "P",
            KeyCode::UpperQ => "Q",
            KeyCode::UpperR => "R",
            KeyCode::UpperS => "S",
            KeyCode::UpperT => "T",
            KeyCode::UpperU => "U",
            KeyCode::UpperV => "V",
            KeyCode::UpperW => "W",
            KeyCode::UpperX => "X",
            KeyCode::UpperY => "Y",
            KeyCode::UpperZ => "Z",
            KeyCode::LeftBracket => "[",
            KeyCode::Backslash => "\\",
            KeyCode::RightBracket => "]",
            KeyCode::Caret => "^",
            KeyCode::Underscore => "_",
            KeyCode::Grave => "`",
            KeyCode::LowerA => "a",
            KeyCode::LowerB => "b",
            KeyCode::LowerC => "c",
            KeyCode::LowerD => "d",
            KeyCode::LowerE => "e",
            KeyCode::LowerF => "f",
            KeyCode::LowerG => "g",
            KeyCode::LowerH => "h",
            KeyCode::LowerI => "i",
            KeyCode::LowerJ => "j",
            KeyCode::LowerK => "k",
            KeyCode::LowerL => "l",
            KeyCode::LowerM => "m",
            KeyCode::LowerN => "n",
            KeyCode::LowerO => "o",
            KeyCode::LowerP => "p",
            KeyCode::LowerQ => "q",
            KeyCode::LowerR => "r",
            KeyCode::LowerS => "s",
            KeyCode::LowerT => "t",
            KeyCode::LowerU => "u",
            KeyCode::LowerV => "v",
            KeyCode::LowerW => "w",
            KeyCode::LowerX => "x",
            KeyCode::LowerY => "y",
            KeyCode::LowerZ => "z",
            KeyCode::LeftBrace => "{",
            KeyCode::Pipe => "|",
            KeyCode::RightBrace => "}",
            KeyCode::Tilde => "~",
            KeyCode::Del => "\x7F",
        }
    }
}

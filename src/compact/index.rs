use std::io::Cursor;

use Result;
use compact::{Offset, OffsetSize, Operations, StringID};
use tape::{Tape, Value, Walue};

table_define! {
    #[doc = "An index."]
    pub Index {
        count       (u16         ),
        offset_size (OffsetSize  ),
        offsets     (Vec<Offset> ),
        data        (Vec<Vec<u8>>),
    }
}

impl Value for Index {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let count = try!(tape.take::<u16>());
        if count == 0 {
            return Ok(Index::default());
        }
        let offset_size = try!(tape.take::<OffsetSize>());
        let mut offsets = Vec::with_capacity(count as usize + 1);
        for i in 0..(count as usize + 1) {
            let offset = try!(Offset::read(tape, offset_size));
            if i == 0 && offset != Offset(1) || i > 0 && offset <= offsets[i - 1] {
                raise!("found a malformed index");
            }
            offsets.push(offset);
        }
        let mut data = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            let size = (u32::from(offsets[i + 1]) - u32::from(offsets[i])) as usize;
            data.push(try!(Walue::read(tape, size)));
        }
        Ok(Index { count: count, offset_size: offset_size, offsets: offsets, data: data })
    }
}

deref! { Index::data => [Vec<u8>] }

macro_rules! index {
    ($(#[$attribute:meta])* pub $structure:ident) => (
        index_define! { $(#[$attribute])* pub $structure }
        index_implement! { $structure }
    );
}

macro_rules! index_define {
    ($(#[$attribute:meta])* pub $structure:ident) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure {
            index: ::compact::Index,
        }

        deref! { $structure::index => ::compact::Index }
    );
}

macro_rules! index_implement {
    ($structure:ident) => (
        impl ::tape::Value for $structure {
            #[inline]
            fn read<T: ::tape::Tape>(tape: &mut T) -> ::Result<Self> {
                Ok($structure { index: try!(::tape::Value::read(tape)) })
            }
        }
    );
}

index_define! {
    #[doc = "A char-strings index."]
    pub CharStrings
}

index! {
    #[doc = "A top-dictionaries index."]
    pub TopDictionaries
}

index! {
    #[doc = "A names index."]
    pub Names
}

index! {
    #[doc = "A strings index."]
    pub Strings
}

index! {
    #[doc = "A subroutines index."]
    pub Subroutines
}

impl Walue<i32> for CharStrings {
    fn read<T: Tape>(tape: &mut T, format: i32) -> Result<Self> {
        Ok(match format {
            2 => CharStrings { index: try!(Value::read(tape)) },
            _ => raise!("found an unknown char-string format"),
        })
    }
}

impl TopDictionaries {
    #[doc(hidden)]
    pub fn into_vec(self) -> Result<Vec<Operations>> {
        let TopDictionaries { index: Index { data, .. } } = self;
        let mut vector = Vec::with_capacity(data.len());
        for chunk in data {
            vector.push(try!(Value::read(&mut Cursor::new(chunk))));
        }
        Ok(vector)
    }
}

impl Names {
    #[doc(hidden)]
    pub fn into_vec(self) -> Result<Vec<String>> {
        let Names { index: Index { data, .. } } = self;
        let mut vector = Vec::with_capacity(data.len());
        for chunk in data {
            vector.push(match String::from_utf8(chunk) {
                Ok(string) => string,
                Err(chunk) => String::from_utf8_lossy(&chunk.into_bytes()).into_owned(),
            });
        }
        Ok(vector)
    }
}

impl Strings {
    /// Return the string corresponding to a string identifier.
    pub fn get(&self, sid: StringID) -> Option<String> {
        match sid as usize {
            i if i < NUMBER_OF_STANDARD_STRINGS => {
                get_standard_string(sid).map(|string| string.to_string())
            },
            i => self.index.get(i - NUMBER_OF_STANDARD_STRINGS).map(|chunk| {
                String::from_utf8_lossy(chunk).into_owned()
            }),
        }
    }
}

const NUMBER_OF_STANDARD_STRINGS: usize = 391;

fn get_standard_string(sid: StringID) -> Option<&'static str> {
    Some(match sid {
        0 => ".notdef",
        1 => "space",
        2 => "exclam",
        3 => "quotedbl",
        4 => "numbersign",
        5 => "dollar",
        6 => "percent",
        7 => "ampersand",
        8 => "quoteright",
        9 => "parenleft",
        10 => "parenright",
        11 => "asterisk",
        12 => "plus",
        13 => "comma",
        14 => "hyphen",
        15 => "period",
        16 => "slash",
        17 => "zero",
        18 => "one",
        19 => "two",
        20 => "three",
        21 => "four",
        22 => "five",
        23 => "six",
        24 => "seven",
        25 => "eight",
        26 => "nine",
        27 => "colon",
        28 => "semicolon",
        29 => "less",
        30 => "equal",
        31 => "greater",
        32 => "question",
        33 => "at",
        34 => "A",
        35 => "B",
        36 => "C",
        37 => "D",
        38 => "E",
        39 => "F",
        40 => "G",
        41 => "H",
        42 => "I",
        43 => "J",
        44 => "K",
        45 => "L",
        46 => "M",
        47 => "N",
        48 => "O",
        49 => "P",
        50 => "Q",
        51 => "R",
        52 => "S",
        53 => "T",
        54 => "U",
        55 => "V",
        56 => "W",
        57 => "X",
        58 => "Y",
        59 => "Z",
        60 => "bracketleft",
        61 => "backslash",
        62 => "bracketright",
        63 => "asciicircum",
        64 => "underscore",
        65 => "quoteleft",
        66 => "a",
        67 => "b",
        68 => "c",
        69 => "d",
        70 => "e",
        71 => "f",
        72 => "g",
        73 => "h",
        74 => "i",
        75 => "j",
        76 => "k",
        77 => "l",
        78 => "m",
        79 => "n",
        80 => "o",
        81 => "p",
        82 => "q",
        83 => "r",
        84 => "s",
        85 => "t",
        86 => "u",
        87 => "v",
        88 => "w",
        89 => "x",
        90 => "y",
        91 => "z",
        92 => "braceleft",
        93 => "bar",
        94 => "braceright",
        95 => "asciitilde",
        96 => "exclamdown",
        97 => "cent",
        98 => "sterling",
        99 => "fraction",
        100 => "yen",
        101 => "florin",
        102 => "section",
        103 => "currency",
        104 => "quotesingle",
        105 => "quotedblleft",
        106 => "guillemotleft",
        107 => "guilsinglleft",
        108 => "guilsinglright",
        109 => "fi",
        110 => "fl",
        111 => "endash",
        112 => "dagger",
        113 => "daggerdbl",
        114 => "periodcentered",
        115 => "paragraph",
        116 => "bullet",
        117 => "quotesinglbase",
        118 => "quotedblbase",
        119 => "quotedblright",
        120 => "guillemotright",
        121 => "ellipsis",
        122 => "perthousand",
        123 => "questiondown",
        124 => "grave",
        125 => "acute",
        126 => "circumflex",
        127 => "tilde",
        128 => "macron",
        129 => "breve",
        130 => "dotaccent",
        131 => "dieresis",
        132 => "ring",
        133 => "cedilla",
        134 => "hungarumlaut",
        135 => "ogonek",
        136 => "caron",
        137 => "emdash",
        138 => "AE",
        139 => "ordfeminine",
        140 => "Lslash",
        141 => "Oslash",
        142 => "OE",
        143 => "ordmasculine",
        144 => "ae",
        145 => "dotlessi",
        146 => "lslash",
        147 => "oslash",
        148 => "oe",
        149 => "germandbls",
        150 => "onesuperior",
        151 => "logicalnot",
        152 => "mu",
        153 => "trademark",
        154 => "Eth",
        155 => "onehalf",
        156 => "plusminus",
        157 => "Thorn",
        158 => "onequarter",
        159 => "divide",
        160 => "brokenbar",
        161 => "degree",
        162 => "thorn",
        163 => "threequarters",
        164 => "twosuperior",
        165 => "registered",
        166 => "minus",
        167 => "eth",
        168 => "multiply",
        169 => "threesuperior",
        170 => "copyright",
        171 => "Aacute",
        172 => "Acircumflex",
        173 => "Adieresis",
        174 => "Agrave",
        175 => "Aring",
        176 => "Atilde",
        177 => "Ccedilla",
        178 => "Eacute",
        179 => "Ecircumflex",
        180 => "Edieresis",
        181 => "Egrave",
        182 => "Iacute",
        183 => "Icircumflex",
        184 => "Idieresis",
        185 => "Igrave",
        186 => "Ntilde",
        187 => "Oacute",
        188 => "Ocircumflex",
        189 => "Odieresis",
        190 => "Ograve",
        191 => "Otilde",
        192 => "Scaron",
        193 => "Uacute",
        194 => "Ucircumflex",
        195 => "Udieresis",
        196 => "Ugrave",
        197 => "Yacute",
        198 => "Ydieresis",
        199 => "Zcaron",
        200 => "aacute",
        201 => "acircumflex",
        202 => "adieresis",
        203 => "agrave",
        204 => "aring",
        205 => "atilde",
        206 => "ccedilla",
        207 => "eacute",
        208 => "ecircumflex",
        209 => "edieresis",
        210 => "egrave",
        211 => "iacute",
        212 => "icircumflex",
        213 => "idieresis",
        214 => "igrave",
        215 => "ntilde",
        216 => "oacute",
        217 => "ocircumflex",
        218 => "odieresis",
        219 => "ograve",
        220 => "otilde",
        221 => "scaron",
        222 => "uacute",
        223 => "ucircumflex",
        224 => "udieresis",
        225 => "ugrave",
        226 => "yacute",
        227 => "ydieresis",
        228 => "zcaron",
        229 => "exclamsmall",
        230 => "Hungarumlautsmall",
        231 => "dollaroldstyle",
        232 => "dollarsuperior",
        233 => "ampersandsmall",
        234 => "Acutesmall",
        235 => "parenleftsuperior",
        236 => "parenrightsuperior",
        237 => "twodotenleader",
        238 => "onedotenleader",
        239 => "zerooldstyle",
        240 => "oneoldstyle",
        241 => "twooldstyle",
        242 => "threeoldstyle",
        243 => "fouroldstyle",
        244 => "fiveoldstyle",
        245 => "sixoldstyle",
        246 => "sevenoldstyle",
        247 => "eightoldstyle",
        248 => "nineoldstyle",
        249 => "commasuperior",
        250 => "threequartersemdash",
        251 => "periodsuperior",
        252 => "questionsmall",
        253 => "asuperior",
        254 => "bsuperior",
        255 => "centsuperior",
        256 => "dsuperior",
        257 => "esuperior",
        258 => "isuperior",
        259 => "lsuperior",
        260 => "msuperior",
        261 => "nsuperior",
        262 => "osuperior",
        263 => "rsuperior",
        264 => "ssuperior",
        265 => "tsuperior",
        266 => "ff",
        267 => "ffi",
        268 => "ffl",
        269 => "parenleftinferior",
        270 => "parenrightinferior",
        271 => "Circumflexsmall",
        272 => "hyphensuperior",
        273 => "Gravesmall",
        274 => "Asmall",
        275 => "Bsmall",
        276 => "Csmall",
        277 => "Dsmall",
        278 => "Esmall",
        279 => "Fsmall",
        280 => "Gsmall",
        281 => "Hsmall",
        282 => "Ismall",
        283 => "Jsmall",
        284 => "Ksmall",
        285 => "Lsmall",
        286 => "Msmall",
        287 => "Nsmall",
        288 => "Osmall",
        289 => "Psmall",
        290 => "Qsmall",
        291 => "Rsmall",
        292 => "Ssmall",
        293 => "Tsmall",
        294 => "Usmall",
        295 => "Vsmall",
        296 => "Wsmall",
        297 => "Xsmall",
        298 => "Ysmall",
        299 => "Zsmall",
        300 => "colonmonetary",
        301 => "onefitted",
        302 => "rupiah",
        303 => "Tildesmall",
        304 => "exclamdownsmall",
        305 => "centoldstyle",
        306 => "Lslashsmall",
        307 => "Scaronsmall",
        308 => "Zcaronsmall",
        309 => "Dieresissmall",
        310 => "Brevesmall",
        311 => "Caronsmall",
        312 => "Dotaccentsmall",
        313 => "Macronsmall",
        314 => "figuredash",
        315 => "hypheninferior",
        316 => "Ogoneksmall",
        317 => "Ringsmall",
        318 => "Cedillasmall",
        319 => "questiondownsmall",
        320 => "oneeighth",
        321 => "threeeighths",
        322 => "fiveeighths",
        323 => "seveneighths",
        324 => "onethird",
        325 => "twothirds",
        326 => "zerosuperior",
        327 => "foursuperior",
        328 => "fivesuperior",
        329 => "sixsuperior",
        330 => "sevensuperior",
        331 => "eightsuperior",
        332 => "ninesuperior",
        333 => "zeroinferior",
        334 => "oneinferior",
        335 => "twoinferior",
        336 => "threeinferior",
        337 => "fourinferior",
        338 => "fiveinferior",
        339 => "sixinferior",
        340 => "seveninferior",
        341 => "eightinferior",
        342 => "nineinferior",
        343 => "centinferior",
        344 => "dollarinferior",
        345 => "periodinferior",
        346 => "commainferior",
        347 => "Agravesmall",
        348 => "Aacutesmall",
        349 => "Acircumflexsmall",
        350 => "Atildesmall",
        351 => "Adieresissmall",
        352 => "Aringsmall",
        353 => "AEsmall",
        354 => "Ccedillasmall",
        355 => "Egravesmall",
        356 => "Eacutesmall",
        357 => "Ecircumflexsmall",
        358 => "Edieresissmall",
        359 => "Igravesmall",
        360 => "Iacutesmall",
        361 => "Icircumflexsmall",
        362 => "Idieresissmall",
        363 => "Ethsmall",
        364 => "Ntildesmall",
        365 => "Ogravesmall",
        366 => "Oacutesmall",
        367 => "Ocircumflexsmall",
        368 => "Otildesmall",
        369 => "Odieresissmall",
        370 => "OEsmall",
        371 => "Oslashsmall",
        372 => "Ugravesmall",
        373 => "Uacutesmall",
        374 => "Ucircumflexsmall",
        375 => "Udieresissmall",
        376 => "Yacutesmall",
        377 => "Thornsmall",
        378 => "Ydieresissmall",
        379 => "001.000",
        380 => "001.001",
        381 => "001.002",
        382 => "001.003",
        383 => "Black",
        384 => "Bold",
        385 => "Book",
        386 => "Light",
        387 => "Medium",
        388 => "Regular",
        389 => "Roman",
        390 => "Semibold",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use compact::StringID;
    use super::NUMBER_OF_STANDARD_STRINGS;
    use super::get_standard_string;

    #[test]
    fn number_of_standard_strings() {
        assert!(get_standard_string(NUMBER_OF_STANDARD_STRINGS as StringID - 1).is_some());
        assert!(get_standard_string(NUMBER_OF_STANDARD_STRINGS as StringID).is_none());
    }
}

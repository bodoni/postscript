//! The encodings.

use crate::compact1::{GlyphID, StringID};

/// An encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Encoding {
    Standard,
    Expert,
}

impl Encoding {
    /// Return the string identifier of a glyph.
    #[inline]
    pub fn get(&self, glyph_id: GlyphID) -> Option<StringID> {
        match *self {
            Encoding::Standard => get_standard(glyph_id),
            Encoding::Expert => get_expert(glyph_id),
        }
    }
}

macro_rules! get(
    ($one:ident { $($glyph_id:pat => $string_id:expr => $name:expr,)+ }) => (
        Some(match $one {
            $($glyph_id => $string_id,)+
            _ => return None,
        })
    );
);

fn get_standard(glyph_id: GlyphID) -> Option<StringID> {
    get!(glyph_id {
        0 => 0 => ".notdef",
        1 => 0 => ".notdef",
        2 => 0 => ".notdef",
        3 => 0 => ".notdef",
        4 => 0 => ".notdef",
        5 => 0 => ".notdef",
        6 => 0 => ".notdef",
        7 => 0 => ".notdef",
        8 => 0 => ".notdef",
        9 => 0 => ".notdef",
        10 => 0 => ".notdef",
        11 => 0 => ".notdef",
        12 => 0 => ".notdef",
        13 => 0 => ".notdef",
        14 => 0 => ".notdef",
        15 => 0 => ".notdef",
        16 => 0 => ".notdef",
        17 => 0 => ".notdef",
        18 => 0 => ".notdef",
        19 => 0 => ".notdef",
        20 => 0 => ".notdef",
        21 => 0 => ".notdef",
        22 => 0 => ".notdef",
        23 => 0 => ".notdef",
        24 => 0 => ".notdef",
        25 => 0 => ".notdef",
        26 => 0 => ".notdef",
        27 => 0 => ".notdef",
        28 => 0 => ".notdef",
        29 => 0 => ".notdef",
        30 => 0 => ".notdef",
        31 => 0 => ".notdef",
        32 => 1 => "space",
        33 => 2 => "exclam",
        34 => 3 => "quotedbl",
        35 => 4 => "numbersign",
        36 => 5 => "dollar",
        37 => 6 => "percent",
        38 => 7 => "ampersand",
        39 => 8 => "quoteright",
        40 => 9 => "parenleft",
        41 => 10 => "parenright",
        42 => 11 => "asterisk",
        43 => 12 => "plus",
        44 => 13 => "comma",
        45 => 14 => "hyphen",
        46 => 15 => "period",
        47 => 16 => "slash",
        48 => 17 => "zero",
        49 => 18 => "one",
        50 => 19 => "two",
        51 => 20 => "three",
        52 => 21 => "four",
        53 => 22 => "five",
        54 => 23 => "six",
        55 => 24 => "seven",
        56 => 25 => "eight",
        57 => 26 => "nine",
        58 => 27 => "colon",
        59 => 28 => "semicolon",
        60 => 29 => "less",
        61 => 30 => "equal",
        62 => 31 => "greater",
        63 => 32 => "question",
        64 => 33 => "at",
        65 => 34 => "A",
        66 => 35 => "B",
        67 => 36 => "C",
        68 => 37 => "D",
        69 => 38 => "E",
        70 => 39 => "F",
        71 => 40 => "G",
        72 => 41 => "H",
        73 => 42 => "I",
        74 => 43 => "J",
        75 => 44 => "K",
        76 => 45 => "L",
        77 => 46 => "M",
        78 => 47 => "N",
        79 => 48 => "O",
        80 => 49 => "P",
        81 => 50 => "Q",
        82 => 51 => "R",
        83 => 52 => "S",
        84 => 53 => "T",
        85 => 54 => "U",
        86 => 55 => "V",
        87 => 56 => "W",
        88 => 57 => "X",
        89 => 58 => "Y",
        90 => 59 => "Z",
        91 => 60 => "bracketleft",
        92 => 61 => "backslash",
        93 => 62 => "bracketright",
        94 => 63 => "asciicircum",
        95 => 64 => "underscore",
        96 => 65 => "quoteleft",
        97 => 66 => "a",
        98 => 67 => "b",
        99 => 68 => "c",
        100 => 69 => "d",
        101 => 70 => "e",
        102 => 71 => "f",
        103 => 72 => "g",
        104 => 73 => "h",
        105 => 74 => "i",
        106 => 75 => "j",
        107 => 76 => "k",
        108 => 77 => "l",
        109 => 78 => "m",
        110 => 79 => "n",
        111 => 80 => "o",
        112 => 81 => "p",
        113 => 82 => "q",
        114 => 83 => "r",
        115 => 84 => "s",
        116 => 85 => "t",
        117 => 86 => "u",
        118 => 87 => "v",
        119 => 88 => "w",
        120 => 89 => "x",
        121 => 90 => "y",
        122 => 91 => "z",
        123 => 92 => "braceleft",
        124 => 93 => "bar",
        125 => 94 => "braceright",
        126 => 95 => "asciitilde",
        127 => 0 => ".notdef",
        128 => 0 => ".notdef",
        129 => 0 => ".notdef",
        130 => 0 => ".notdef",
        131 => 0 => ".notdef",
        132 => 0 => ".notdef",
        133 => 0 => ".notdef",
        134 => 0 => ".notdef",
        135 => 0 => ".notdef",
        136 => 0 => ".notdef",
        137 => 0 => ".notdef",
        138 => 0 => ".notdef",
        139 => 0 => ".notdef",
        140 => 0 => ".notdef",
        141 => 0 => ".notdef",
        142 => 0 => ".notdef",
        143 => 0 => ".notdef",
        144 => 0 => ".notdef",
        145 => 0 => ".notdef",
        146 => 0 => ".notdef",
        147 => 0 => ".notdef",
        148 => 0 => ".notdef",
        149 => 0 => ".notdef",
        150 => 0 => ".notdef",
        151 => 0 => ".notdef",
        152 => 0 => ".notdef",
        153 => 0 => ".notdef",
        154 => 0 => ".notdef",
        155 => 0 => ".notdef",
        156 => 0 => ".notdef",
        157 => 0 => ".notdef",
        158 => 0 => ".notdef",
        159 => 0 => ".notdef",
        160 => 0 => ".notdef",
        161 => 96 => "exclamdown",
        162 => 97 => "cent",
        163 => 98 => "sterling",
        164 => 99 => "fraction",
        165 => 100 => "yen",
        166 => 101 => "florin",
        167 => 102 => "section",
        168 => 103 => "currency",
        169 => 104 => "quotesingle",
        170 => 105 => "quotedblleft",
        171 => 106 => "guillemotleft",
        172 => 107 => "guilsinglleft",
        173 => 108 => "guilsinglright",
        174 => 109 => "fi",
        175 => 110 => "fl",
        176 => 0 => ".notdef",
        177 => 111 => "endash",
        178 => 112 => "dagger",
        179 => 113 => "daggerdbl",
        180 => 114 => "periodcentered",
        181 => 0 => ".notdef",
        182 => 115 => "paragraph",
        183 => 116 => "bullet",
        184 => 117 => "quotesinglbase",
        185 => 118 => "quotedblbase",
        186 => 119 => "quotedblright",
        187 => 120 => "guillemotright",
        188 => 121 => "ellipsis",
        189 => 122 => "perthousand",
        190 => 0 => ".notdef",
        191 => 123 => "questiondown",
        192 => 0 => ".notdef",
        193 => 124 => "grave",
        194 => 125 => "acute",
        195 => 126 => "circumflex",
        196 => 127 => "tilde",
        197 => 128 => "macron",
        198 => 129 => "breve",
        199 => 130 => "dotaccent",
        200 => 131 => "dieresis",
        201 => 0 => ".notdef",
        202 => 132 => "ring",
        203 => 133 => "cedilla",
        204 => 0 => ".notdef",
        205 => 134 => "hungarumlaut",
        206 => 135 => "ogonek",
        207 => 136 => "caron",
        208 => 137 => "emdash",
        209 => 0 => ".notdef",
        210 => 0 => ".notdef",
        211 => 0 => ".notdef",
        212 => 0 => ".notdef",
        213 => 0 => ".notdef",
        214 => 0 => ".notdef",
        215 => 0 => ".notdef",
        216 => 0 => ".notdef",
        217 => 0 => ".notdef",
        218 => 0 => ".notdef",
        219 => 0 => ".notdef",
        220 => 0 => ".notdef",
        221 => 0 => ".notdef",
        222 => 0 => ".notdef",
        223 => 0 => ".notdef",
        224 => 0 => ".notdef",
        225 => 138 => "AE",
        226 => 0 => ".notdef",
        227 => 139 => "ordfeminine",
        228 => 0 => ".notdef",
        229 => 0 => ".notdef",
        230 => 0 => ".notdef",
        231 => 0 => ".notdef",
        232 => 140 => "Lslash",
        233 => 141 => "Oslash",
        234 => 142 => "OE",
        235 => 143 => "ordmasculine",
        236 => 0 => ".notdef",
        237 => 0 => ".notdef",
        238 => 0 => ".notdef",
        239 => 0 => ".notdef",
        240 => 0 => ".notdef",
        241 => 144 => "ae",
        242 => 0 => ".notdef",
        243 => 0 => ".notdef",
        244 => 0 => ".notdef",
        245 => 145 => "dotlessi",
        246 => 0 => ".notdef",
        247 => 0 => ".notdef",
        248 => 146 => "lslash",
        249 => 147 => "oslash",
        250 => 148 => "oe",
        251 => 149 => "germandbls",
        252 => 0 => ".notdef",
        253 => 0 => ".notdef",
        254 => 0 => ".notdef",
        255 => 0 => ".notdef",
    })
}

fn get_expert(glyph_id: GlyphID) -> Option<StringID> {
    get!(glyph_id {
        0 => 0 => ".notdef",
        1 => 0 => ".notdef",
        2 => 0 => ".notdef",
        3 => 0 => ".notdef",
        4 => 0 => ".notdef",
        5 => 0 => ".notdef",
        6 => 0 => ".notdef",
        7 => 0 => ".notdef",
        8 => 0 => ".notdef",
        9 => 0 => ".notdef",
        10 => 0 => ".notdef",
        11 => 0 => ".notdef",
        12 => 0 => ".notdef",
        13 => 0 => ".notdef",
        14 => 0 => ".notdef",
        15 => 0 => ".notdef",
        16 => 0 => ".notdef",
        17 => 0 => ".notdef",
        18 => 0 => ".notdef",
        19 => 0 => ".notdef",
        20 => 0 => ".notdef",
        21 => 0 => ".notdef",
        22 => 0 => ".notdef",
        23 => 0 => ".notdef",
        24 => 0 => ".notdef",
        25 => 0 => ".notdef",
        26 => 0 => ".notdef",
        27 => 0 => ".notdef",
        28 => 0 => ".notdef",
        29 => 0 => ".notdef",
        30 => 0 => ".notdef",
        31 => 0 => ".notdef",
        32 => 1 => "space",
        33 => 229 => "exclamsmall",
        34 => 230 => "Hungarumlautsmall",
        35 => 0 => ".notdef",
        36 => 231 => "dollaroldstyle",
        37 => 232 => "dollarsuperior",
        38 => 233 => "ampersandsmall",
        39 => 234 => "Acutesmall",
        40 => 235 => "parenleftsuperior",
        41 => 236 => "parenrightsuperior",
        42 => 237 => "twodotenleader",
        43 => 238 => "onedotenleader",
        44 => 13 => "comma",
        45 => 14 => "hyphen",
        46 => 15 => "period",
        47 => 99 => "fraction",
        48 => 239 => "zerooldstyle",
        49 => 240 => "oneoldstyle",
        50 => 241 => "twooldstyle",
        51 => 242 => "threeoldstyle",
        52 => 243 => "fouroldstyle",
        53 => 244 => "fiveoldstyle",
        54 => 245 => "sixoldstyle",
        55 => 246 => "sevenoldstyle",
        56 => 247 => "eightoldstyle",
        57 => 248 => "nineoldstyle",
        58 => 27 => "colon",
        59 => 28 => "semicolon",
        60 => 249 => "commasuperior",
        61 => 250 => "threequartersemdash",
        62 => 251 => "periodsuperior",
        63 => 252 => "questionsmall",
        64 => 0 => ".notdef",
        65 => 253 => "asuperior",
        66 => 254 => "bsuperior",
        67 => 255 => "centsuperior",
        68 => 256 => "dsuperior",
        69 => 257 => "esuperior",
        70 => 0 => ".notdef",
        71 => 0 => ".notdef",
        72 => 0 => ".notdef",
        73 => 258 => "isuperior",
        74 => 0 => ".notdef",
        75 => 0 => ".notdef",
        76 => 259 => "lsuperior",
        77 => 260 => "msuperior",
        78 => 261 => "nsuperior",
        79 => 262 => "osuperior",
        80 => 0 => ".notdef",
        81 => 0 => ".notdef",
        82 => 263 => "rsuperior",
        83 => 264 => "ssuperior",
        84 => 265 => "tsuperior",
        85 => 0 => ".notdef",
        86 => 266 => "ff",
        87 => 109 => "fi",
        88 => 110 => "fl",
        89 => 267 => "ffi",
        90 => 268 => "ffl",
        91 => 269 => "parenleftinferior",
        92 => 0 => ".notdef",
        93 => 270 => "parenrightinferior",
        94 => 271 => "Circumflexsmall",
        95 => 272 => "hyphensuperior",
        96 => 273 => "Gravesmall",
        97 => 274 => "Asmall",
        98 => 275 => "Bsmall",
        99 => 276 => "Csmall",
        100 => 277 => "Dsmall",
        101 => 278 => "Esmall",
        102 => 279 => "Fsmall",
        103 => 280 => "Gsmall",
        104 => 281 => "Hsmall",
        105 => 282 => "Ismall",
        106 => 283 => "Jsmall",
        107 => 284 => "Ksmall",
        108 => 285 => "Lsmall",
        109 => 286 => "Msmall",
        110 => 287 => "Nsmall",
        111 => 288 => "Osmall",
        112 => 289 => "Psmall",
        113 => 290 => "Qsmall",
        114 => 291 => "Rsmall",
        115 => 292 => "Ssmall",
        116 => 293 => "Tsmall",
        117 => 294 => "Usmall",
        118 => 295 => "Vsmall",
        119 => 296 => "Wsmall",
        120 => 297 => "Xsmall",
        121 => 298 => "Ysmall",
        122 => 299 => "Zsmall",
        123 => 300 => "colonmonetary",
        124 => 301 => "onefitted",
        125 => 302 => "rupiah",
        126 => 303 => "Tildesmall",
        127 => 0 => ".notdef",
        128 => 0 => ".notdef",
        129 => 0 => ".notdef",
        130 => 0 => ".notdef",
        131 => 0 => ".notdef",
        132 => 0 => ".notdef",
        133 => 0 => ".notdef",
        134 => 0 => ".notdef",
        135 => 0 => ".notdef",
        136 => 0 => ".notdef",
        137 => 0 => ".notdef",
        138 => 0 => ".notdef",
        139 => 0 => ".notdef",
        140 => 0 => ".notdef",
        141 => 0 => ".notdef",
        142 => 0 => ".notdef",
        143 => 0 => ".notdef",
        144 => 0 => ".notdef",
        145 => 0 => ".notdef",
        146 => 0 => ".notdef",
        147 => 0 => ".notdef",
        148 => 0 => ".notdef",
        149 => 0 => ".notdef",
        150 => 0 => ".notdef",
        151 => 0 => ".notdef",
        152 => 0 => ".notdef",
        153 => 0 => ".notdef",
        154 => 0 => ".notdef",
        155 => 0 => ".notdef",
        156 => 0 => ".notdef",
        157 => 0 => ".notdef",
        158 => 0 => ".notdef",
        159 => 0 => ".notdef",
        160 => 0 => ".notdef",
        161 => 304 => "exclamdownsmall",
        162 => 305 => "centoldstyle",
        163 => 306 => "Lslashsmall",
        164 => 0 => ".notdef",
        165 => 0 => ".notdef",
        166 => 307 => "Scaronsmall",
        167 => 308 => "Zcaronsmall",
        168 => 309 => "Dieresissmall",
        169 => 310 => "Brevesmall",
        170 => 311 => "Caronsmall",
        171 => 0 => ".notdef",
        172 => 312 => "Dotaccentsmall",
        173 => 0 => ".notdef",
        174 => 0 => ".notdef",
        175 => 313 => "Macronsmall",
        176 => 0 => ".notdef",
        177 => 0 => ".notdef",
        178 => 314 => "figuredash",
        179 => 315 => "hypheninferior",
        180 => 0 => ".notdef",
        181 => 0 => ".notdef",
        182 => 316 => "Ogoneksmall",
        183 => 317 => "Ringsmall",
        184 => 318 => "Cedillasmall",
        185 => 0 => ".notdef",
        186 => 0 => ".notdef",
        187 => 0 => ".notdef",
        188 => 158 => "onequarter",
        189 => 155 => "onehalf",
        190 => 163 => "threequarters",
        191 => 319 => "questiondownsmall",
        192 => 320 => "oneeighth",
        193 => 321 => "threeeighths",
        194 => 322 => "fiveeighths",
        195 => 323 => "seveneighths",
        196 => 324 => "onethird",
        197 => 325 => "twothirds",
        198 => 0 => ".notdef",
        199 => 0 => ".notdef",
        200 => 326 => "zerosuperior",
        201 => 150 => "onesuperior",
        202 => 164 => "twosuperior",
        203 => 169 => "threesuperior",
        204 => 327 => "foursuperior",
        205 => 328 => "fivesuperior",
        206 => 329 => "sixsuperior",
        207 => 330 => "sevensuperior",
        208 => 331 => "eightsuperior",
        209 => 332 => "ninesuperior",
        210 => 333 => "zeroinferior",
        211 => 334 => "oneinferior",
        212 => 335 => "twoinferior",
        213 => 336 => "threeinferior",
        214 => 337 => "fourinferior",
        215 => 338 => "fiveinferior",
        216 => 339 => "sixinferior",
        217 => 340 => "seveninferior",
        218 => 341 => "eightinferior",
        219 => 342 => "nineinferior",
        220 => 343 => "centinferior",
        221 => 344 => "dollarinferior",
        222 => 345 => "periodinferior",
        223 => 346 => "commainferior",
        224 => 347 => "Agravesmall",
        225 => 348 => "Aacutesmall",
        226 => 349 => "Acircumflexsmall",
        227 => 350 => "Atildesmall",
        228 => 351 => "Adieresissmall",
        229 => 352 => "Aringsmall",
        230 => 353 => "AEsmall",
        231 => 354 => "Ccedillasmall",
        232 => 355 => "Egravesmall",
        233 => 356 => "Eacutesmall",
        234 => 357 => "Ecircumflexsmall",
        235 => 358 => "Edieresissmall",
        236 => 359 => "Igravesmall",
        237 => 360 => "Iacutesmall",
        238 => 361 => "Icircumflexsmall",
        239 => 362 => "Idieresissmall",
        240 => 363 => "Ethsmall",
        241 => 364 => "Ntildesmall",
        242 => 365 => "Ogravesmall",
        243 => 366 => "Oacutesmall",
        244 => 367 => "Ocircumflexsmall",
        245 => 368 => "Otildesmall",
        246 => 369 => "Odieresissmall",
        247 => 370 => "OEsmall",
        248 => 371 => "Oslashsmall",
        249 => 372 => "Ugravesmall",
        250 => 373 => "Uacutesmall",
        251 => 374 => "Ucircumflexsmall",
        252 => 375 => "Udieresissmall",
        253 => 376 => "Yacutesmall",
        254 => 377 => "Thornsmall",
        255 => 378 => "Ydieresissmall",
    })
}

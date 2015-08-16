use compact::primitive::*;

table! {
    #[derive(Copy)]
    pub Header {
        major   (u8        ),
        minor   (u8        ),
        hdrSize (u8        ),
        offSize (OffsetSize),
    }
}

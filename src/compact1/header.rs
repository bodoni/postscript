use crate::compact1::OffsetSize;

table! {
    #[doc = "A header."]
    #[derive(Copy)]
    pub Header {
        major       (u8        ), // major
        minor       (u8        ), // minor
        header_size (u8        ), // hdrSize
        offset_size (OffsetSize), // offSize
    }
}

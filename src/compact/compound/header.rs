use compact::primitive::OffsetSize;

table! {
    #[doc = "A header."]
    #[derive(Copy)]
    pub Header {
        major       (u8        ),
        minor       (u8        ),
        header_size (u8        ),
        offset_size (OffsetSize),
    }
}

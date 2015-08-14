use compact::primitive::*;

spec! {
    #[derive(Copy)]
    pub Header {
        major   (Card8  ),
        minor   (Card8  ),
        hdrSize (Card8  ),
        offSize (OffSize),
    }
}

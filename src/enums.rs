pub mod enums {
    #![allow(unused_variables)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
}

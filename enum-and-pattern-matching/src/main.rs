fn main() {
    enum IpAddrkind {
        V4(String),
        V6(String)
    }


    let home = IpAddrkind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrkind::V6(String::from("::1"));
}

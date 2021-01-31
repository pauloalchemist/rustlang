fn main() {
    enum IpAddrkind {
        V4,
        V6
    }

    struct IpAddr {
        kind: IpAddrkind,
        address: String
    }

    let home = IpAddr {
        kind: IpAddrkind::V4,
        address: String::from("127.0.0.1")
    };


    let loopback = IpAddr {
        kind: IpAddrkind::V6,
        address: String::from("::1")
    };
}

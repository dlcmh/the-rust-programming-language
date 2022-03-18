fn main() {
    // Define an enumeration that enumerates all possible variants of the 2 major standards used for IP addresses
    #[derive(Debug)]
    enum IpAddrKind {
        V4, // version 4
        V6, // version 6
    }

    // Create instance of a variant
    let v4 = IpAddrKind::V4;

    // define a function that takes any `IpAddrKind`
    fn route(ip_kind: IpAddrKind) {
        println!("got: {:?}", ip_kind);
    }

    // route("v4"); // got: V4
    // error[E0308]: mismatched types
    //   --> src/main.rs:18:11
    //    |
    // 18 |     route("v4"); // got: V4
    //    |           ^^^^ expected enum `IpAddrKind`, found `&str`
    route(v4); // got: V4
    route(IpAddrKind::V6); // got: V6
}

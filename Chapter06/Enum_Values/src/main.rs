fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
fn route(ip_type: IpAddrKind) {}
route(IpAddrKind::V4);
route(IpAddrKind::V6);
}

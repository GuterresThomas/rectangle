enum IpAddrKind {
    v4,
    v6,
}



fn main () {
    let four = IpAddrKind::v4;
    let six: IpAddrKind = IpAddrKind::v6;

    route(IpAddrKind::v4);
    route(IpAddrKind::v6);

}

fn route(ip_kind: IpAddrKind) {}
use get_if_addrs;

fn main() {
    for iface in get_if_addrs::get_if_addrs().unwrap() {
        println!("{}: {}", iface.name, iface.addr.ip());
    }
}

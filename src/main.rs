mod version;

fn main() {
    println!("Hello, world!");

    let os_version = version::get_os_version();
    println!("{:#?}", os_version);
}

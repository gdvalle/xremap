use zbus::blocking::Connection;
fn main() {
    let conn = Connection::session().unwrap();
}

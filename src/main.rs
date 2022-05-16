mod syndb;

fn main() {
    let mut db = syndb::Database::new("db.json", true);
    db.load();
    db.set("name", "John-117");
    db.save();
}

mod syndb;

fn main() {
    let mut db = syndb::Database::new("db.json", true);
    db.load();
    db.set("key", "value");
    db.save();
    db.exit();
}

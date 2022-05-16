mod syndb;

fn main() {
    let mut db = syndb::Database::new("db.json", true);
    db.load();
    db.set("name", "John-117");
    db.set("age", "50");
    db.save();
}

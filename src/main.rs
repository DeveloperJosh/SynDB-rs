mod db;

fn main() {
    let mut db = db::Database::new("db.json", false);
    db.load();
    db.set("name", "John-117");
    db.save();
}

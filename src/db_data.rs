use rbtree::RBTree;

use byte_unit::n_kb_bytes;

pub const MAX_SIZE: u128  = n_kb_bytes!(32);

#[cfg(test)]
mod tests {
    use crate::db_data;
    use db_data::DBData;

    #[test]
    fn it_works()  {
        let mut db = DBData::new();
        let foo: String = String::from("foo");
        let bar: String = String::from("bar");

        let overflow = db.write(foo, bar);
        let result = db.get(String::from("foo")).unwrap();

        assert_eq!(overflow, false);
        assert_eq!(*result, String::from("bar"));
    }

    #[test]
    fn it_knows_when_its_full()  {
        let mut db = DBData::new();
        let foo: String = String::from("foo");
        let max_size_string = "a".repeat(db_data::MAX_SIZE as usize);
        let bar: String = String::from(max_size_string);

        let overflow = db.write(foo, bar);

        assert_eq!(overflow, true);
        // assert_eq!(*result, String::from("bar"));
    }

}

pub struct DBData { memtable: RBTree<String, String>, pub size: u128, max_size: u128}
impl DBData {
    pub fn new() -> DBData {
        let tree: RBTree<String, String> = RBTree::new();
        let max_size = MAX_SIZE;
        return DBData {memtable: tree, size: 0, max_size};
    }

    pub fn write(& mut self, k: String, v: String) -> bool {
        self.size += v.len() as u128;
        self.memtable.insert(k, v);
        return DBData::check_overflow(self.size, self.max_size);
    }

    fn check_overflow(size: u128, max_size: u128) -> bool {
        return size >= max_size;
    }

    fn get(&self, k: String) -> Option<&String> {
        return self.memtable.get(&k);
    }
}


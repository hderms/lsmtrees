 use crate::db_data;
 use db_data::DBData;

 #[cfg(test)]
 mod tests {
     use crate::db;
     use crate::db_data;
     use db::DB;
     #[test]
     fn it_gets_an_empty_db_after_overflow()  {
         let mut db = DB::new();
         let foo: String = String::from("foo");
         let max_size_string = "a".repeat(db_data::MAX_SIZE as usize);
         let bar: String = String::from(max_size_string);

         let db = db.write(foo, bar);

         assert_eq!(db.empty(), true);
     }
 }
struct DB { db: DBData}
impl DB {
    fn new() -> DB {
        DB{db: DBData::new()}
    }
    fn write(mut self, k: String, v: String) -> DB  {
        return if self.db.write(k, v) {
            DB{db: DBData::new()}
        } else {
            self
        }
    }

    fn empty(self) -> bool {
        self.db.size == 0
    }
}

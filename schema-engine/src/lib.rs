use std::collections::{HashMap, HashSet};

use smol_str::SmolStr;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub struct DbSchemaName(SmolStr);
pub struct DbTableName(SmolStr);

pub struct SchemaEngine {
    main_db: DbSchema,
    attached_dbs: Vec<DbSchema>,
}

pub struct DbSchema {
    name: DbSchemaName,
    tables: HashMap<DbTableName, DbTable>,
}

//
pub struct DbTable {
    is_strict: bool,
}

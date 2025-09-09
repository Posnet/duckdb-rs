use super::{OrderedMap, ToSql, ToSqlOutput, Value};
use crate::Result;
use std::collections::HashMap;
use std::hash::Hash;

/// Generic implementation of ToSql for HashMap where both K and V can be converted to Value
impl<K, V> ToSql for HashMap<K, V>
where
    K: Clone + Into<Value> + Eq + Hash,
    V: Clone + Into<Value>,
{
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::new();
        for (k, v) in self.iter() {
            pairs.push((k.clone().into(), v.clone().into()));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}
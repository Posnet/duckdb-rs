use super::{OrderedMap, ToSql, ToSqlOutput, Value};
use crate::Result;
use std::collections::HashMap;

// Optimized implementations for common HashMap types
// Key optimizations:
// 1. Pre-allocate Vec capacity to avoid reallocations
// 2. Direct construction of Value types without generic overhead
// 3. For numeric types, just copy the value (cheap)
// 4. For String, we must clone (unavoidable as Value owns the data)

/// Optimized implementation for HashMap<String, String> - most common case
impl ToSql for HashMap<String, String> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::Text(v.clone())));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

/// Optimized implementation for HashMap<String, i32>
impl ToSql for HashMap<String, i32> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::Int(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

/// Optimized implementation for HashMap<String, i64>
impl ToSql for HashMap<String, i64> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::BigInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

/// Optimized implementation for HashMap<String, f64>
impl ToSql for HashMap<String, f64> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::Double(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

/// Optimized implementation for HashMap<String, f32>
impl ToSql for HashMap<String, f32> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::Float(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

/// Optimized implementation for HashMap<String, bool>
impl ToSql for HashMap<String, bool> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::Boolean(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

/// Implementation for HashMap<i32, String>
impl ToSql for HashMap<i32, String> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Int(*k), Value::Text(v.clone())));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

/// Implementation for HashMap<i32, i32>
impl ToSql for HashMap<i32, i32> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Int(*k), Value::Int(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

/// Implementation for HashMap<i32, f64>
impl ToSql for HashMap<i32, f64> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Int(*k), Value::Double(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

// Additional numeric type implementations
impl ToSql for HashMap<String, u32> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::UInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<String, u64> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::UBigInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<String, i8> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::TinyInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<String, i16> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::SmallInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<String, i128> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::HugeInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<String, u8> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::UTinyInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<String, u16> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::Text(k.clone()), Value::USmallInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<i64, String> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::BigInt(*k), Value::Text(v.clone())));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<i64, i64> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::BigInt(*k), Value::BigInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<u8, i128> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::UTinyInt(*k), Value::HugeInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}

impl ToSql for HashMap<i16, u64> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let mut pairs = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            pairs.push((Value::SmallInt(*k), Value::UBigInt(*v)));
        }
        Ok(ToSqlOutput::Owned(Value::Map(OrderedMap::from(pairs))))
    }
}
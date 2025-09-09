use crate::ffi;
use crate::types::{OrderedMap, Value};

/// Helper to create a DuckDB value from Rust Value
pub(super) unsafe fn create_duckdb_value(value: &Value) -> Option<ffi::duckdb_value> {
    match value {
        Value::Null => {
            Some(ffi::duckdb_create_null_value())
        }
        Value::Boolean(b) => {
            Some(ffi::duckdb_create_bool(*b))
        }
        Value::TinyInt(i) => {
            Some(ffi::duckdb_create_int8(*i))
        }
        Value::SmallInt(i) => {
            Some(ffi::duckdb_create_int16(*i))
        }
        Value::Int(i) => {
            Some(ffi::duckdb_create_int32(*i))
        }
        Value::BigInt(i) => {
            Some(ffi::duckdb_create_int64(*i))
        }
        Value::HugeInt(i) => {
            let hugeint = ffi::duckdb_hugeint {
                lower: *i as u64,
                upper: (*i >> 64) as i64,
            };
            Some(ffi::duckdb_create_hugeint(hugeint))
        }
        Value::UTinyInt(i) => {
            Some(ffi::duckdb_create_uint8(*i))
        }
        Value::USmallInt(i) => {
            Some(ffi::duckdb_create_uint16(*i))
        }
        Value::UInt(i) => {
            Some(ffi::duckdb_create_uint32(*i))
        }
        Value::UBigInt(i) => {
            Some(ffi::duckdb_create_uint64(*i))
        }
        Value::Float(f) => {
            Some(ffi::duckdb_create_float(*f))
        }
        Value::Double(d) => {
            Some(ffi::duckdb_create_double(*d))
        }
        Value::Text(s) => {
            let c_str = std::ffi::CString::new(s.as_str()).ok()?;
            Some(ffi::duckdb_create_varchar(c_str.as_ptr()))
        }
        Value::Map(map) => {
            create_map_value(map)
        }
        _ => None, // Other types (Blob, Date32, Time64, Interval, List, Enum, Struct, Array, Union) not implemented yet
    }
}

/// Create a MAP value from an OrderedMap
unsafe fn create_map_value(map: &OrderedMap<Value, Value>) -> Option<ffi::duckdb_value> {
    let entries: Vec<_> = map.iter().collect();
    if entries.is_empty() {
        // Create an empty MAP
        // First need to determine the key and value types
        // For empty maps, we'll use VARCHAR for both
        let mut key_type = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR);
        let mut value_type = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR);
        let mut map_type = ffi::duckdb_create_map_type(key_type, value_type);
        
        // Empty vectors for empty map
        let mut keys: Vec<ffi::duckdb_value> = Vec::new();
        let mut values: Vec<ffi::duckdb_value> = Vec::new();
        
        let result = ffi::duckdb_create_map_value(
            map_type,
            keys.as_mut_ptr(),
            values.as_mut_ptr(),
            0
        );
        
        ffi::duckdb_destroy_logical_type(&mut key_type);
        ffi::duckdb_destroy_logical_type(&mut value_type);
        ffi::duckdb_destroy_logical_type(&mut map_type);
        
        Some(result)
    } else {
        // Create arrays of keys and values
        let mut keys: Vec<ffi::duckdb_value> = Vec::new();
        let mut values: Vec<ffi::duckdb_value> = Vec::new();
        
        // Determine types from first entry
        let (first_key, first_value) = entries[0];
        let mut key_type = get_logical_type(first_key)?;
        let mut value_type = get_logical_type(first_value)?;
        
        // Convert all entries
        for (k, v) in &entries {
            let key_val = create_duckdb_value(k)?;
            let value_val = create_duckdb_value(v)?;
            keys.push(key_val);
            values.push(value_val);
        }
        
        // Create the map type
        let mut map_type = ffi::duckdb_create_map_type(key_type, value_type);
        
        // Create the map value
        let result = ffi::duckdb_create_map_value(
            map_type,
            keys.as_mut_ptr(),
            values.as_mut_ptr(),
            entries.len() as ffi::idx_t
        );
        
        // Clean up
        for mut key in keys {
            ffi::duckdb_destroy_value(&mut key);
        }
        for mut value in values {
            ffi::duckdb_destroy_value(&mut value);
        }
        ffi::duckdb_destroy_logical_type(&mut key_type);
        ffi::duckdb_destroy_logical_type(&mut value_type);
        ffi::duckdb_destroy_logical_type(&mut map_type);
        
        Some(result)
    }
}

/// Get the logical type for a Value
unsafe fn get_logical_type(value: &Value) -> Option<ffi::duckdb_logical_type> {
    match value {
        Value::Null => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_INVALID)),
        Value::Boolean(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_BOOLEAN)),
        Value::TinyInt(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_TINYINT)),
        Value::SmallInt(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_SMALLINT)),
        Value::Int(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_INTEGER)),
        Value::BigInt(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_BIGINT)),
        Value::HugeInt(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_HUGEINT)),
        Value::UTinyInt(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_UTINYINT)),
        Value::USmallInt(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_USMALLINT)),
        Value::UInt(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_UINTEGER)),
        Value::UBigInt(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_UBIGINT)),
        Value::Float(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_FLOAT)),
        Value::Double(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_DOUBLE)),
        Value::Text(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR)),
        Value::Blob(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_BLOB)),
        Value::Map(_) => Some(ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_MAP)),
        _ => None, // Other types (Date32, Time64, Interval, List, Enum, Struct, Array, Union, Decimal) not supported yet
    }
}
use duckdb::{Connection, params};
use std::collections::HashMap;

#[test]
fn test_generic_map_support() -> Result<(), duckdb::Error> {
    let conn = Connection::open_in_memory()?;
    
    // Create a table with various MAP column types
    conn.execute(
        "CREATE TABLE test_generic_maps (
            id INTEGER,
            string_to_string MAP(VARCHAR, VARCHAR),
            string_to_int MAP(VARCHAR, INTEGER),
            string_to_float MAP(VARCHAR, DOUBLE),
            int_to_string MAP(INTEGER, VARCHAR),
            int_to_int MAP(INTEGER, INTEGER)
        )",
        []
    )?;
    
    // Test various HashMap type combinations
    {
        let mut appender = conn.appender("test_generic_maps")?;
        
        // HashMap<String, String>
        let mut string_map = HashMap::new();
        string_map.insert("key1".to_string(), "value1".to_string());
        string_map.insert("key2".to_string(), "value2".to_string());
        
        // HashMap<String, i32>
        let mut string_int_map = HashMap::new();
        string_int_map.insert("count".to_string(), 42i32);
        string_int_map.insert("size".to_string(), 100i32);
        
        // HashMap<String, f64>
        let mut string_float_map = HashMap::new();
        string_float_map.insert("temperature".to_string(), 23.5f64);
        string_float_map.insert("pressure".to_string(), 1013.25f64);
        
        // HashMap<i32, String>
        let mut int_string_map = HashMap::new();
        int_string_map.insert(1i32, "first".to_string());
        int_string_map.insert(2i32, "second".to_string());
        
        // HashMap<i32, i32>
        let mut int_int_map = HashMap::new();
        int_int_map.insert(10i32, 100i32);
        int_int_map.insert(20i32, 200i32);
        
        appender.append_row(params![
            1,
            &string_map,
            &string_int_map,
            &string_float_map,
            &int_string_map,
            &int_int_map
        ])?;
        
        // Test with empty maps of different types
        let empty_string_map: HashMap<String, String> = HashMap::new();
        let empty_int_map: HashMap<String, i64> = HashMap::new();
        let empty_float_map: HashMap<String, f32> = HashMap::new();
        let empty_int_string: HashMap<i64, String> = HashMap::new();
        let empty_int_int: HashMap<u32, u32> = HashMap::new();
        
        appender.append_row(params![
            2,
            &empty_string_map,
            &empty_int_map,
            &empty_float_map,
            &empty_int_string,
            &empty_int_int
        ])?;
        
        appender.flush()?;
    }
    
    // Verify the data
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM test_generic_maps",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(count, 2);
    
    // Test accessing different map types
    let string_val: Option<String> = conn.query_row(
        "SELECT string_to_string['key1'] FROM test_generic_maps WHERE id = 1",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(string_val, Some("value1".to_string()));
    
    let int_val: Option<i32> = conn.query_row(
        "SELECT string_to_int['count'] FROM test_generic_maps WHERE id = 1",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(int_val, Some(42));
    
    let float_val: Option<f64> = conn.query_row(
        "SELECT string_to_float['temperature'] FROM test_generic_maps WHERE id = 1",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(float_val, Some(23.5));
    
    println!("✅ All generic MAP type combinations work!");
    
    Ok(())
}

#[test]
fn test_map_with_various_numeric_types() -> Result<(), duckdb::Error> {
    let conn = Connection::open_in_memory()?;
    
    conn.execute(
        "CREATE TABLE test_numeric_maps (
            id INTEGER,
            u8_map MAP(VARCHAR, UTINYINT),
            u16_map MAP(VARCHAR, USMALLINT),
            u32_map MAP(VARCHAR, UINTEGER),
            u64_map MAP(VARCHAR, UBIGINT),
            i8_map MAP(VARCHAR, TINYINT),
            i16_map MAP(VARCHAR, SMALLINT),
            f32_map MAP(VARCHAR, FLOAT),
            i128_map MAP(VARCHAR, HUGEINT)
        )",
        []
    )?;
    
    {
        let mut appender = conn.appender("test_numeric_maps")?;
        
        // Test all numeric types
        let mut u8_map = HashMap::new();
        u8_map.insert("max".to_string(), 255u8);
        
        let mut u16_map = HashMap::new();
        u16_map.insert("max".to_string(), 65535u16);
        
        let mut u32_map = HashMap::new();
        u32_map.insert("max".to_string(), u32::MAX);
        
        let mut u64_map = HashMap::new();
        u64_map.insert("max".to_string(), u64::MAX);
        
        let mut i8_map = HashMap::new();
        i8_map.insert("min".to_string(), i8::MIN);
        
        let mut i16_map = HashMap::new();
        i16_map.insert("min".to_string(), i16::MIN);
        
        let mut f32_map = HashMap::new();
        f32_map.insert("pi".to_string(), 3.14159f32);
        
        let mut i128_map = HashMap::new();
        i128_map.insert("huge".to_string(), i128::MAX);
        
        appender.append_row(params![
            1,
            &u8_map,
            &u16_map,
            &u32_map,
            &u64_map,
            &i8_map,
            &i16_map,
            &f32_map,
            &i128_map
        ])?;
        
        appender.flush()?;
    }
    
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM test_numeric_maps",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(count, 1);
    
    println!("✅ All numeric MAP types work!");
    
    Ok(())
}
use duckdb::{Connection, params};
use std::collections::HashMap;

#[test]
fn test_map_appender() -> Result<(), duckdb::Error> {
    let conn = Connection::open_in_memory()?;
    
    // Create a table with a MAP column
    conn.execute(
        "CREATE TABLE test_map (
            id INTEGER,
            attributes MAP(VARCHAR, VARCHAR)
        )",
        []
    )?;
    
    // Test with appender
    {
        let mut appender = conn.appender("test_map")?;
        
        // Empty map
        let empty_map: HashMap<String, String> = HashMap::new();
        appender.append_row(params![1, &empty_map])?;
        
        // Single entry
        let mut single_map = HashMap::new();
        single_map.insert("key".to_string(), "value".to_string());
        appender.append_row(params![2, &single_map])?;
        
        // Multiple entries
        let mut multi_map = HashMap::new();
        multi_map.insert("name".to_string(), "test".to_string());
        multi_map.insert("type".to_string(), "example".to_string());
        appender.append_row(params![3, &multi_map])?;
        
        appender.flush()?;
    }
    
    // Verify the data
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM test_map",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(count, 3);
    
    Ok(())
}

#[test]
fn test_map_bulk_insert() -> Result<(), duckdb::Error> {
    let conn = Connection::open_in_memory()?;
    
    conn.execute(
        "CREATE TABLE metrics (
            timestamp INTEGER,
            tags MAP(VARCHAR, VARCHAR),
            values MAP(VARCHAR, DOUBLE)
        )",
        []
    )?;
    
    {
        let mut appender = conn.appender("metrics")?;
        
        for i in 0..100 {
            let mut tags = HashMap::new();
            tags.insert("host".to_string(), format!("server-{}", i % 10));
            tags.insert("service".to_string(), "api".to_string());
            
            let mut values = HashMap::new();
            values.insert("cpu".to_string(), (i as f64) * 0.5);
            values.insert("memory".to_string(), (i as f64) * 100.0);
            
            appender.append_row(params![i, &tags, &values])?;
        }
        
        appender.flush()?;
    }
    
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM metrics",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(count, 100);
    
    Ok(())
}
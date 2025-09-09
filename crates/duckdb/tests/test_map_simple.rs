use duckdb::{Connection, params};
use std::collections::HashMap;

#[test]
fn test_map_simple() -> Result<(), duckdb::Error> {
    let conn = Connection::open_in_memory()?;
    
    // Create a table with a MAP column
    conn.execute(
        "CREATE TABLE test_map (
            id INTEGER,
            attributes MAP(VARCHAR, VARCHAR)
        )",
        []
    )?;
    
    // Test with a single simple entry
    let mut map = HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    
    println!("Attempting to append map: {:?}", map);
    
    // Use appender
    {
        let mut appender = conn.appender("test_map")?;
        appender.append_row(params![1, &map])?;
        appender.flush()?;
    }
    
    println!("Map appended successfully!");
    
    Ok(())
}
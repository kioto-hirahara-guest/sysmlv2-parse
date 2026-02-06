use syster::hir::{RootDatabase, FileText, parse_file, file_symbols};
use syster::base::FileId;

fn main() {
    // Create the Salsa database
    let db = RootDatabase::new();

    // Set file content (input query)
    let file_id = FileId::new(0);
    let file_text = FileText::new(&db, file_id, r#"
        package Vehicle {
            part def Car {
                attribute mass : Real;
            }
        }
    "#.to_string());

    // Parse (memoized - subsequent calls are instant)
    let parse_result = parse_file(&db, file_text);
    assert!(parse_result.is_ok());

    // Extract symbols (also memoized)
    if let Some(ast) = parse_result.get_syntax_file() {
        let symbols = file_symbols(file_id, ast);
        // symbols contains: Vehicle (package), Car (part def), mass (attribute)
        dbg!(&symbols);
    }
}

use std::env;
use std::fs;
use syster::hir::{RootDatabase, FileText, parse_file, file_symbols};
use syster::base::FileId;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }
    let contents = fs::read_to_string(args[1].clone())
        .expect("No such file or directory");

    // Create the Salsa database
    let db = RootDatabase::new();

    // Set file content (input query)
    let file_id = FileId::new(0);
    let file_text: FileText = FileText::new(&db, file_id, contents);

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

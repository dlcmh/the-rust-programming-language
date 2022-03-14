fn main() {
    // # Store multiple types with an Enum

    // We want to get values from a row in a spreadsheet in which some columns
    // contain integers, some floating-point numbers, and some strings. We can
    // define an enum whose variants will hold different value types, and then
    // all the enum variants will be considered the same type: that of the enum.

    #[derive(Debug)] // to allow println! to work
    enum SpreadsheetCell {
        // `SpreadsheetCell` doesn't implement `Debug`
        // the trait `Debug` is not implemented for `SpreadsheetCell`
        // add `#[derive(Debug)]` to `SpreadsheetCell` or manually `impl Debug for SpreadsheetCell` rustc(E0277)
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {:?}", row); // row: [Int(3), Text("blue"), Float(10.12)]

    // Rust needs to know what types will be in the vector at compile time so it
    // knows exactly how much memory on the heap will be needed to store each
    // element.

    // An additional advantage is we can be explicit about what types are
    // allowed in this vector. This reduces the risk of one or more types
    // causing errors with operations performed on the elements of the vector.

    // Use a trait object if the exhaustive set of types the program will get
    // at truntime aren't known.
}

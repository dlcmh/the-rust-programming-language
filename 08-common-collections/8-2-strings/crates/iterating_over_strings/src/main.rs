use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s_str1 = "大卫";
    let s_string = String::from("大卫");
    let hindi_str = "नमस्ते"; // Hindi

    println!("{}", s_str1);
    // 大卫

    // the `chars` method separates out and returns individual Unicode scalar values
    println!("{:?}", s_str1.chars());
    // Chars(['大', '卫'])
    println!("{:?}", s_string.chars());
    // Chars(['大', '卫'])

    // Hindi - The 6 characters in "नमस्ते" do not map 1:1 with the 4 grapheme clusters (the closest thing to "letters")
    println!("{:?}", hindi_str.chars());
    // Chars(['न', 'म', 'स', '\u{94d}', 'त', '\u{947}'])
    // the `\u..` characters are diacritics  ् and  े respectively
    for c in hindi_str.chars() {
        println!("{}", c);
    }
    // see output at https://doc.rust-lang.org/book/ch08-02-strings.html#methods-for-iterating-over-strings as the diacritics aren't visible whan pasted in here: े
    // (copied from the HTML source):
    // न
    // म
    // स
    // \u{94d} ्
    // त
    // \u{947} े

    let hindi_clusters = hindi_str.graphemes(true).collect::<Vec<&str>>();
    println!("{:?}", hindi_clusters);
    // ["न", "म", "स\u{94d}", "त\u{947}"]
    for g in hindi_str.graphemes(true) {
        println!("{}", g);
    }
    // न
    // म
    // स्
    // ते
    println!(
        "Count of clusters in {} is {:?}",
        hindi_str,
        hindi_clusters.len()
    );
    // Count of clusters in नमस्ते is 4
    // In Ruby irb (https://www.linkedin.com/pulse/grapheme-clusters-ruby-english-example-sourav-goswami/):
    // "नमस्ते".grapheme_clusters.length # => 4

    println!("{:?}", s_str1.bytes());
    // Bytes(Copied { it: Iter([229, 164, 167, 229, 141, 171]) })
    println!("{:?}", s_string.bytes());
    // Bytes(Copied { it: Iter([229, 164, 167, 229, 141, 171]) })
}

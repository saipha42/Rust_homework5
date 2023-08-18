
fn main() {

    let qoute = "  C ** *C++* *Java *Python* Rust*    ";

    let result = extract_quoted_words(qoute, Vec::new());
    println!("Result of qoutes : {:?}", result);
}

fn extract_quoted_words(qoute : &str, mut result : Vec<String>)-> Vec<String> {

    let qoute_string = qoute.to_string();
    let words: Vec<&str> = qoute_string.split_whitespace().collect();
    
    //base case
    if words.len() == 0 {
        return result;
    }

    if words[0].len() != 1 && words[0].starts_with("*") && words[0].ends_with("*") {
        
        let qoute_word = words[0];
        result.push(qoute_word[1..qoute_word.len() -1].to_string());
    }
    extract_quoted_words(words[1..].join(" ").as_str(), result.clone())
    
}

#[cfg(test)]

#[test]
fn test_extract_quoted_words_recursion() {
    assert_eq!(extract_quoted_words("", Vec::new()), Vec::<String>::new() );
    assert_eq!(
    extract_quoted_words("C ** *C++* *Java *Python* Rust*", Vec::new()),
        ["", "C++", "Python"] // "**", "*C++*", "*Python*"
    );
}

#[test]
fn test_extract_quoted_words_recursion_extra_cases() {

    assert_eq!(extract_quoted_words("   ", Vec::new() ), Vec::<String>::new());
    assert_eq!(extract_quoted_words("  * ", Vec::new() ), Vec::<String>::new());

    let test_string = String::from("*Javascript* *A *Rust* *C '' *TA* **   *Python*   ");

    let res = extract_quoted_words(&test_string, Vec::new());

    assert_eq!(res, vec!["Javascript", "Rust", "TA", "","Python"]);
}
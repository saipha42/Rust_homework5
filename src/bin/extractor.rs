
fn main() {

    let qoute = "C ** *C++* *Java *Python* Rust* ";
    let result = extract_quoted_words(qoute);

    println!("Result of qoute : {:?}", result);
}

fn extract_quoted_words(qoute : &str) -> Vec<String> {

    let qoute = qoute.to_string();
    let qoute = qoute.split(" ");
    let mut result = Vec::new();

    for word in qoute{

        let open_star = word.chars().next().unwrap_or('_');
        let close_star = word.chars().rev().next().unwrap_or('_');
        
        if open_star == '*' && close_star == '*' {

            let (_, word) = word.split_at(1);
            let (word, _) = word.split_at(word.len()-1);

            result.push(word.to_string());
        }

    }

    return result;
}


#[cfg(test)]
#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new() );
    assert_eq!(
    extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
        ["", "C++", "Python"] // "**", "*C++*", "*Python*"
    );
}


#[test]
fn test_extract_quoted_words_extra_cases() {

    assert_eq!(extract_quoted_words(""), Vec::<String>::new() );
    assert_eq!(
    extract_quoted_words("abc *jarvis* *i* *s* *Tony's* *assistance*    **"),
        ["jarvis","i","s","Tony's", "assistance", ""]
    );
}


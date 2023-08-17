
fn main() {

    let qoute = String::from("C ** *C++* *Java *Python* Rust*");
    extract_quoted_words(qoute);
}


fn extract_quoted_words(qoute : String) {

    let qoute = qoute.split(" ");
    let mut result = Vec::new();

    for word in qoute{

        let open_star = word.chars().next().unwrap();
        let close_star = word.chars().rev().next().unwrap();
        
        if open_star == '*' && close_star == '*' {

            let (star, word) = word.split_at(1);
            let (word, star) = word.split_at(word.len()-1);

            result.push(word);
        }

    }

    dbg!(result);
}



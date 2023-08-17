
fn main() {

    let qoute = "*C*   ** *C++*h *Java* *Python*  ";

    let result = extract_quoted_words(qoute, Vec::new());
    println!("Result of qoutes : {:?}", result);
}

#[derive(Debug)]
struct Space {
    index : usize,
    found : bool
}

fn extract_quoted_words(qoute : &str, mut result : Vec<String>)-> Vec<String> {

    let qoute = qoute.to_string();
    let space_index = qoute.find(" ");

    let space_index = match space_index {
        Some(idx) => Space {index : idx, found: true},
        None => Space { index: 0, found: false }
    };

    //base case for stopping the recusion
    if space_index.found == false {
        return result;
    }

    let mut  next_index = space_index.index;
    if space_index.index == 0 {
        next_index = space_index.index + 1;
    }
    let (pre_word, rest_words) = qoute.split_at(next_index);


    //checking for the qoute
    let open_star = pre_word.chars().next().unwrap();
    let close_star = pre_word.chars().rev().next().unwrap();
    
    if open_star == '*' && close_star == '*' {

        //remove first star and last star of the word
        let (_, word) = pre_word.split_at(1);
        let (word, _) = word.split_at(word.len()-1);

        result.push(word.to_string());
    }

    //check for the last work in the sentence if it is a qoute
    if !rest_words.contains(" ") && rest_words !="" {

        let open_star = rest_words.chars().next().unwrap();
        let close_star = rest_words.chars().rev().next().unwrap();
        
        if open_star == '*' && close_star == '*' {
    
            //remove first star and last star of the word
            let (_, word) = rest_words.split_at(1);
            let (word, _) = word.split_at(word.len()-1);
    
            result.push(word.to_string());
        }
    }

    extract_quoted_words(rest_words, result)

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

    let test_string = String::from("*Javascript* *A *Rust* *C '' *TA* **   *Python*   ");

    let res = extract_quoted_words(&test_string, Vec::new());

    assert_eq!(res, vec!["Javascript", "Rust", "TA", "","Python"]);
}
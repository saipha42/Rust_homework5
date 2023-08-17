
fn main() {

    let qoute = String::from("*C*   ** *C++*h **Java** *Python is good* Rust*");

    let res = extract_quoted_words(qoute, Vec::new());
    println!("{:?}", res);
}

#[derive(Debug)]
struct Space {
    index : usize,
    found : bool
}

fn extract_quoted_words(mut qoute : String, mut result : Vec<String>)-> Vec<String> {

    let space_index = qoute.find(" ");

    let space_index = match space_index {
        Some(idx) => Space {index : idx, found: true},
        None => Space { index: 0, found: false }
    };

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
        let (star, word) = pre_word.split_at(1);
        let (word, star) = word.split_at(word.len()-1);

        result.push(word.to_string());
    }

    extract_quoted_words(rest_words.to_string(), result)

}
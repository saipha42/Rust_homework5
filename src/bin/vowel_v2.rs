
fn main() {

    let input_string = "ttt ttt  ttattt   ";

    let result = count_vowels_v2(&input_string);
    println!("Number of vowels : {:?}", result);
}


fn count_vowels_v2(vowel_string: &str ) -> Vec<(&str, i32)> {

    if vowel_string.is_empty() {
        return Vec::new();
    }
    
    let mut result: Vec<(&str, i32)> = Vec::new();
    let vowels = vec!["a", "e", "i", "o", "u", "A", "E", "I", "o", "U"];


    for word in vowel_string.split(" ") {

        let mut  vw_count = 0;
        for ch in word.chars() {
        
            if vowels.contains(&ch.to_string().as_str()) {
               
                vw_count += 1;
            }
        }

        //check to not include whitespace in the list
        if word != "" {
            result.push(( word, vw_count  ));
        }
        
    }

    return  result;

}


#[cfg(test)]
#[test]
fn test_vowels_count_v2() {
    assert_eq!(count_vowels_v2(""), []);
    assert_eq!(
        count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
            ("ab12Exey5", 3),
            ("7x8U3y5z", 1) 
        ]
    );
}

#[test]
fn test_vowels_count_v2_extra_cases() {

    assert_eq!(
        count_vowels_v2("ab12Exey5"),
        [
            ("ab12Exey5", 3), // 'a', 'E', 'e'
        ]
    );

    assert_eq!(
        count_vowels_v2("  aaa bbb ccc fff  "),
        [
            ("aaa", 3),
            ("bbb", 0),
            ("ccc", 0),
            ("fff", 0)

        ]
    );
}
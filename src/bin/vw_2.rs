
fn main() {

    let input_string = "jdfjei erjerj";

    let result = count_vowels_v2(&input_string);


    println!("Number of vowels : {:?}", result);
}


fn count_vowels_v2(vowel_string: &str ) -> Vec<(String, i32)> {

    if vowel_string.is_empty() {
        return Vec::new();
    }
    
    let mut result: Vec<(String, i32)> = Vec::new();
    let vw_vec = vec!["a", "e", "i", "o", "u", "A", "E", "I", "o", "U"];


    for word in vowel_string.split(" ") {
        let mut  vw_count = 0;
        for ch in word.chars() {
        
            if vw_vec.contains(&ch.to_string().as_str()) {
               
                vw_count += 1;
            }
        }

        result.push(( word.to_string(), vw_count  ));
    }


    


    
    return  result;

}



#[test]
fn test_vowels_count2() {
    assert_eq!(count_vowels_v2(""), []);
    assert_eq!(
        count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
            ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
            ("7x8U3y5z".to_string(), 1) // 'U'
        ]
    );
    assert_eq!(
        count_vowels_v2("ab12Exey5"),
        [
            ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
        ]
    );
}
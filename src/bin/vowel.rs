
fn main() {

    let input_string = "ab12Exey5 7x8U3y5z";

    let result = count_vowels(&input_string);
    println!("Number of vowels : {}", result);
}


fn count_vowels(vowel_string: &str ) -> i32 {

    let vowels: Vec<&str> = vec!["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];

    let vw_chars = vowel_string.chars();
    let mut  vw_count = 0;

    for ch in vw_chars {
        
        if vowels.contains(&ch.to_string().as_str()) {

            vw_count += 1;
        }
    }
    
    vw_count
}


#[cfg(test)]
#[test]
fn test_vowels_count() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);

}

#[test]
fn test_vowels_count_extra_cases() {
    
    assert_eq!(count_vowels("ab12Exey5"), 3);
    assert_eq!(count_vowels("ab12Exey5 ab12Exey5 aeiou"), 11);

    assert_eq!(count_vowels("xyz 12345789"), 0);
}
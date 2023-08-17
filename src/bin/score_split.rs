fn main() {

    let result = split_score_to_grade(vec![50,60,80,90]);
    println!("{:?}", result);
}

fn split_score_to_grade(score_list : Vec<i32>)-> (Vec<(String, i32)>, Vec<(String, i32)>) {

    let mut pass_grades = Vec::new();
    let mut fail_grades = Vec::new();

    for score in score_list {
        //the scores that are less than 0 and greater than 100 are not valid and so not listed
       if score >=0 && score <= 49  {
        fail_grades.push( ("F".to_string(), score) );
       }else if score >=50 && score <= 60 {
           fail_grades.push( ("D".to_string(), score) );
       }else if score >= 61 && score <= 70 {
           pass_grades.push( ("C".to_string(), score));
       }
       else if score >= 71 && score <= 80 {
            pass_grades.push( ("B".to_string(), score));
        }else if score >= 81 && score <= 94 {
            pass_grades.push( ("A".to_string(), score));
        }else if score >= 95 && score <= 100 {
            pass_grades.push( ("A+".to_string(), score) )
        }
    }
    
    (pass_grades, fail_grades)
    
}


#[test]
fn test_split_score_to_grade() {
    assert_eq!(split_score_to_grade(vec![]), (vec![], vec![]));
    assert_eq!(
        split_score_to_grade(vec![10, 50, 60, 80, 100, 101]),
        (
            vec![("B".to_string(), 80), ("A+".to_string(), 100)],
            vec![("F".to_string(), 10), ("D".to_string(), 50),("D".to_string(), 60)]
        )
    );
    assert_eq!(
        split_score_to_grade(vec![-10,105]),
        (
            vec![],
            vec![]
        )
    );
}
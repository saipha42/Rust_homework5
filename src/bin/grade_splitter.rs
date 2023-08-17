fn main() {

    let res = split_grade(vec!["A", "A+", "F", "D"]);
    println!("{:?}", res);
}

fn split_grade(grade_list : Vec<&str>)-> (Vec<&str>, Vec<&str>) {

    let valid_grade = vec!["A+", "A", "B", "C","D", "F"];

    let mut pass_grades = Vec::new();
    let mut fail_grades = Vec::new();

    for grade in grade_list {
        
        if valid_grade.contains(&grade) {
            if grade == "D" || grade == "F" {
                fail_grades.push(grade);
            }else {
                pass_grades.push(grade);
            }
        }
    }
    
    (pass_grades, fail_grades)
    
}


#[test]
fn test_split_grade() {
    assert_eq!(split_grade(vec![]), (vec![], vec![]));
    assert_eq!(
        split_grade(vec!["A", "B", "A+", "C", "F", "D", "G"]),
        (
            vec!["A", "B", "A+", "C"],
            vec!["F", "D"]
        )
    );
}
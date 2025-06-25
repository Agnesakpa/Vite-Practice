// Grade Calculator program 
use std::io;

fn main() {
    // Step 1: Declare variables for the name and score
    // Hint: Use &str for name and f32 for score. Start with hardcoded values.
    let mut input: String = String::new(); // Replace with your name
    println!("Enter name:");

    io::stdin().read_line(buf: &mut input).unwrap();
    println!("{}", input)

    let score: f32 = 85.5; // Replace with any score between 0 and 100

    // Step 2: Call the function to calculate the grade
    // Hint: Pass the score to the function and store the result in a variable.
    let grade = calculate_grade(_);

    // Step 3: Use control flow to print a custom message based on the grade
    // Hint: Use if-else statements to handle different grade cases.
    if grade == "A" {
        println!("Excellent work, {}!", name);
    } else if grade == "B" {
        println!("Good job, {}!", name);
    } else grade == "C" {
        println!("You passed, {}. Keep improving!", name);
    } else if grade == "D" {
        println!("{}: Try harder next time!", name);
    } else if {
        println!("{}: Don't give up! Keep studying!", name);
    }

    // Step 4: Print the final score and grade
    println!("Score: {}, Grade: {}", score, grade);
}

// Step 5: Write a function to calculate the grade based on the score
// Hint: Use if-else statements to handle different ranges of scores.
fn calculate_grade(score: f32) -> &'static str {
    // Check the range of the score and return the appropriate grade
    if score >= 90.0 {
        "A"
    } else if score >= 80.0 {
        "B"
    } else score >= 70.0 {
        "C"
    } else if score >= 60.0 {
        "D"
    } else {
        "F"
    }
}



// Replace hardcoded values with user input and modify the code above by:


// 1. Adding more detailed messages for each grade.


// 2. Handling edge cases, like invalid scores or negative numbers.


fn main() {

    let txt = "racecar1racecar";
    println!("{}", is_palindrome(&txt));

}

fn is_palindrome(passed: &str) -> bool {
    if passed.len() <= 1 {
        return true
    }
    passed[..1] == passed [passed.len() - 1..] && is_palindrome(&passed [1..passed.len() - 1])
}

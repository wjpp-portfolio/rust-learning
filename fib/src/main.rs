fn main() {
    fib(1844674407370955165);
}
///genereate fibonacci numbers in order from 0 to the target
fn fib(target: u64) {

    let mut seq = (0, 1);

    println!("{}", seq.0);

    while seq.1 < target {
        
        println!("{}", seq.1);
        seq = (seq.1, seq.0 + seq.1);
                
    }
   
    
}
fn main() {
    println!("September Review of Rust!");
    println!("--------- ------ -- -----");

    println!("\n## Control Flow");
    println!("### If Statements");
    let mut number=10; //define variable as mutable. 

    println!("round 1: {number}");
    if number<=5 {
        println!("Under Five");
    } else {
        println!("Over 5");
    }

    number=4;
   println!("round 2: {number}");
    if number<=5 {
        println!("Under Five");
    } else {
        println!("Over 5");
    }
    println!("\n### Loops"); //Mostly a straight copy-and-paste from an earlier program.
    let mut counter=0;

    let result=loop {
        counter+=1;
        print!("{counter}...");
        if counter==10 {
            break counter*2;
        }
    };

    println!("\nThe result is {result}");
}

use rand::Rng; // Import the Rng trait

fn main() {
    let mut rng = rand::thread_rng(); // Get a thread-local random number generator

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

        println!("\n#### Loop with a Label"); //also cut-and-paste
    counter=0;
    'counting_up: loop {
        println!("Counter={counter}");
        let mut remaining=10;
        loop {
            if counter==2 {
                break 'counting_up;
            }
            println!("\tRemaining={remaining}");
            if remaining==9 { //was 0, but changed to 9 just to keep the output small. I get the point. 
                break;
            }

            remaining -=1;
        }
        counter+=1;
    }
        println!("#### While Loops");
        let mut number=10;

        while number!=0 {
            print!("{number}...");
            number-=1;
        }
        println!("DONE!");

        println!("\n#### For Loops");
        let mut cats=["Beso","Luna","Nami"]; //May need to double back to arrays a bit...actually it doesn't look
                                             //I did much more with arrays than this. Which is fine. 

        for kitty in cats {
            println!("Cat is {kitty}");
        }
       
        println!("\n## Functions");
        println!("Is 6 over or under 5?");
        overfive(6);
        println!("How about 2?");
        overfive(2);
        //going nuts: let's use a random number.
        let mut newnumb=rng.gen_range(1..20);
        println!("Rolling a D20...let's try {newnumb}");
        overfive(newnumb);

    }

    fn overfive(x: i32) {
        if x<=5 {
            println!("\tUnder Five");
        } else {
            println!("\tOver Five");
        }
    }

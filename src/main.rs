fn main() {
    //this just prints "AGANE!!" until stopped
    /*
    loop{
        println!("AGANE!!");
    }
    */

    //counts down from 5 using while loop
    /*
    let mut number = 5;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("B L A S T O F F !");
    */
    
    //for loop through an array

    let a = [11, 22, 33, 44, 55];
    let mut counter = 0;

    for element in a.iter() {
        println!("the value in position {} is: {}", counter, element);
        counter += 1;
    }
}

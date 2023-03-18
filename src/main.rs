fn main() {
    println!("Hello, world!");


    //call function
    main1();

    main2(23);

    let b = sum(1,2);
    println!("The sum = {}", b);
}

fn main1(){
    println!("Welcome to main1 function");
}

fn main2(a:i32){
    println!("What value am I getting = {}", a);
}

fn sum(a:i32, b:i32) -> i32{
    //let z = a + b;
    //return z*10;
    return a + b;
}
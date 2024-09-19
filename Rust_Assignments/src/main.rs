const FREEZE: f64 = 32.0;
fn main() {
    // Assignment 1:
    let mut f: f64 = 10.2;
    let mut fnext: f64 = f;
    f = fahrenheit_to_celsius(f);
    println!("Fahrenheit to Celsius: {}°F = {}°C", fnext, f);

    // loop to convert and print the next 5 integer temperatures
    println!("Next 5 temperatures converted to celsius:");
    for x in 0..5{
        fnext += 1.0;
        println!("{}°F = {}°C,",fnext, fahrenheit_to_celsius(fnext));
    }


    // Assignment 2:
    let num: [i32; 10] = [5, 9, 85, 94, 37, 4, 83 , 56, 75, 29];
    for x in 0..num.len() {
        if is_even(num[x]){
            println!("{} is even", num[x]); }
        if num[x]%3 == 0{
            if num[x]%5 == 0{
                println!("FizzBuzz")
            }
            println!("Fizz")
        }
        if num[x]%5 == 0{
            println!("Buzz") }
    }

    let mut counter = 0;
    let mut sum = 0;
    while counter != 10 {
        sum = sum + num[counter];
        counter += 1;
    }
    println!("Sum: {}",sum);

    let mut biggest_num = 0;
    let mut i = 0;
    loop{
        if num[i] > biggest_num{
            biggest_num = num[i];
        }
        if i == 9 { break;
        }
        i += 1;
    }
    println!("Largest number: {}",biggest_num);

    // Assignment 3:
    
    let mut secret = 86;
    let mut guess = 86;
    let mut count = 1;
    let mut check;
    loop{
      check = check_guess(guess, secret);
    if check == 0 {
        println!("You guessed correctly!");
        break;
    }
    else if check == 1 {
        println!("You guessed too high!");
    }
    else {
        println!("You guessed too low!");
    }  
    count += 1;
    }
    println!("You guessed {} times",count); 
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    // formula °C = (°F - 32) × 5/9
    if f == FREEZE {
        return 0.0;
    }
    return (f - 32.0) * (5 as f64 / 9 as f64);
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
     //formula °F = °C × (9/5) + 32
    if c == 0.0 {
        return FREEZE;
    }
    return c * (9 as f64 / 5 as f64) + 32.0;
}

fn is_even(n: i32) -> bool{
    if n%2 == 0 {
        return true;
    }
    return false;
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {   // return 0 if guess is correct
        return 0;
    }
    else if guess > secret { // return 1 if the guess is too high
        return 1;
    }  
    else{ // return -1 if the guess is too low
        return -1;
    }
}
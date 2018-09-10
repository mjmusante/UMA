mod guessmearray;

use std::process;
use std::io::{self, Write};
use guessmearray::GuessMeArray;

fn main() -> io::Result<()> {

    print!("(H)ost game, or (P)lay game? ");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    match buf.trim_right().as_ref() {
        "P" | "p" => { play_game()?; }
        "H" | "h" => { host_game()?; }
        _ => { println!("invalid [{}]", buf); }
    }

    Ok(())
}

fn host_game() -> io::Result<()> {
    let ground_truth = 4321;
    println!("I have thought of a number for you to guess. Let's play!");

    let mut nguesses = 0;
    loop {
        print!("What's your guess? Input a 4-digit integer: ");

        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        buf = buf.trim_right().to_string();
        let guess = buf.parse::<u16>().unwrap();

        if guess < 1000 || guess > 9999 {
            println!("Your number is out of range!");
            continue;
        }

        nguesses += 1;
        let nmatches = GuessMeArray::num_matches(&guess, &ground_truth);
        if nmatches == 4 {
            print!("You have won! ");
            break;
        }

        println!("Almost there. Number of matches: {}", nmatches);
    }
    println!("The number i had was {}", ground_truth);
    println!("You got it in {} rounds.", nguesses);

    Ok(())
}

fn play_game() -> io::Result<()> {
    let mut gamer = GuessMeArray::new();
    let mut buf = String::new();

    println!("Think of a number between 1000 and 9999\n\
        Press RETURN when ready -");
    io::stdin().read_line(&mut buf)?;

    let mut guess = 0;
    while !gamer.is_over() {
        guess = gamer.get_guess();
        println!("I guess your number is {}", guess);

        print!("How many digits are correct? ");
        io::stdout().flush()?;
        buf = String::new();
        io::stdin().read_line(&mut buf)?;
        buf = buf.trim_right().to_string();

        let nmatches = buf.parse::<u16>().unwrap();
        if nmatches > 4 {
            println!("Invalid. Please enter a number between 0 and 4");
            continue;
        }

        if gamer.update_guess(nmatches) == false {
            println!("Something is wrong. I don't think your number exists");
            process::exit(0);
        }
    }

    println!("I got it. Your number was {}.", guess);
    println!("I did it in {} rounds. Here is the list of my guesses",
        gamer.num_guesses());

    for i in gamer.prior_guesses().iter() {
        if *i > 0 {
            print!("{} ", *i);
        }
    }
    println!("");

    Ok(())
}

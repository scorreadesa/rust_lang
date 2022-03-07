use std::io::Write;
use rand::Rng;

fn main()
{
    println!("========== Guessing Game ==========");

    // Choose difficulty
    print!("Easy (1) - Normal (2) - Hard (3)> ");
    std::io::stdout().flush().unwrap();
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line.");
    choice.pop();

    // Generate random number based on difficulty
    let secret_number;
    let mut rng = rand::thread_rng();
    match choice.as_str() {
        "1" => secret_number = rng.gen_range(0..10),
        "2" => secret_number = rng.gen_range(0..50),
        "3" => secret_number = rng.gen_range(0..100),
        _ => secret_number = rng.gen_range(0..50),
    }

    // Game loop
    let mut n_try = 0;
    let max_tries = 10;
    let found = false;
    let mut guess = String::new();

    while !found && n_try < max_tries
    {
        n_try = n_try + 1;
        guess.clear();
        print!("guess> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line.");
        guess.pop();
        let guessed_number = guess.parse::<i32>().unwrap();

        if guessed_number < secret_number
        {
            println!("The secret number is greater than {}.", guess);
        }
        else if guessed_number > secret_number
        {
            println!("The secret number is smaller than {}.", guess);
        }
        else
        {
            break;
        }
    }

    if n_try < max_tries
    {
        println!("You win ({} attempts): the secret number was {}.", n_try, secret_number);
    }
    else
    {
        println!("You lost ({} attempts): the secret number was {}.", n_try, secret_number);
    }

    println!("Good Bye!");
}

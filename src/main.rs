use std::io;

fn main() {
    let possible_answers = vec![
        "Maybe if you try harder.",
        "Sure, if you believe in fairy tales.",
        "Ha! Good luck with that.",
        "I don't think so.",
        "I'm not your personal genie.",
        "Do I look like a fortune teller to you?",
        "You can wish all you want, but it won't happen.",
        "Might as well ask a rock for the answer.",
        "I don't have time for your nonsense.",
    ];

    loop {
        println!("What is your question for the magical eight ball?");

        let mut question = String::new();
        io::stdin()
            .read_line(&mut question)
            .expect("Failed to read line");

        let answer = possible_answers[rand::random::<usize>() % possible_answers.len()];
        println!("The magical eight ball says: {}", answer);

        println!("Ask another question? (y/n)");

        let mut repeat = String::new();
        io::stdin()
            .read_line(&mut repeat)
            .expect("Failed to read line");

        if repeat.trim() != "y" {
            break;
        }
    }
}

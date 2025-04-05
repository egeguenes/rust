struct Test {
    score: i32,
}

fn main() {
    let colors = vec!["red", "green", "blue"];
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.pop();

    println!("{:?}", numbers[0]);
    println!("{:?}", colors[0]);

    for number in numbers {
        println!("{:?}", number);
    }

    let scores = vec![
        Test { score: 71 },
        Test { score: 93 },
        Test { score: 92 },
        Test { score: 11 },
    ];

    for score in scores {
        println!("Score of the student is {}", score.score);
    }
}

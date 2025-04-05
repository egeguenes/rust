struct Test {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let answer = Test {
        q1: Some(357),
        q2: Some(true),
        q3: None,
    };

    match answer.q1 {
        Some(ans) => println!("Answer is {:?}", ans),
        None => (),
    }

    match answer.q2 {
        Some(ans) => println!("Answer is {:?}", ans),
        None => (),
    }

    match answer.q3 {
        Some(ans) => println!("Answer is {:?}", ans),
        None => println!("No answer"),
    }

    let stud = Student {
        name: "StudentName".to_owned(),
        locker: Some(13),
    };

    println!("Student name: {:?}", stud.name);

    match stud.locker {
        Some(locker_num) => println!("Num is {:?}", locker_num),
        None => println!("No locker num"),
    }
}

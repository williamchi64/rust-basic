fn main() {
    // let a = String::from("hello");
    // let b = a;
    // println!("{}", a);

    // let a = "hello";
    // let b = a;
    // println!("{}", b);

    // let a = String::from("hello");
    // do_sth(a);
    // println!("{}", a);

    // let a = String::from("hello");
    // let b = do_sth(a);
    // println!("{}", b);

    // let a = String::from("hello");
    // let b = &a;
    // print_sth(b);

    // let mut c = String::from("hello");
    // change(&mut c);
    // println!("{}",c);

    // lifetime
    // {
    //     let r;
    
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    
    //     println!("r: {}", r);
    // }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // println!("The longest string is {}", result);
}

fn do_sth(str: String) -> String {
    str
}

fn print_sth(str: &String) {
    let s = String::from("hello");
    // * run copy trait
    assert_eq!(s, *str);
    println!("{:p}", str);
    // let c = *str;
    // println!("{}", c);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
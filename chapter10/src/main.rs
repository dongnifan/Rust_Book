struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point1<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point1<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point1<X2, Y2>) -> Point1<X1, Y2> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point{ x : 5, y: 10};
    let float = Point{x: 1.0, y: 4.0};
    // let wont_work = Point{ x: 5, y: 4.0};

    let both_integer = Point1{ x : 5, y: 10};
    let both_float = Point1{x: 1.0, y: 4.0};
    let integer_and_float = Point1{ x: 5, y: 4.0};

    let p = Point{ x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point1{ x: 5, y: 10.4};
    let p2 = Point1{ x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = Some(5);
    let float = Some(5.0);

    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
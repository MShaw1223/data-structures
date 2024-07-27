use std::collections::HashMap;

fn main() {
    use_array();
    use_object();
    use_hash_map();
}
fn use_array() {
    let array_nums: [i8; 5] = [1, 2, 3, 4, 5];
    for n in array_nums.iter() {
        println!("{}", n)
    }
}
fn use_object() {
    struct Person {
        name: String,
        age: Option<i8>,
    }
    impl Person {
        fn greeting(&self) {
            println!("Hi my name is {}", self.name)
        }
        fn how_old(&self) {
            match self.age {
                Some(age) => println!("I am {}", age),
                None => println!("No age provided..."),
            }
        }
    }
    let person_eg = Person {
        name: String::from("John"),
        age: Some(i8::from(23)),
    };
    let sec_person_eg = Person {
        name: String::from("Foobar"),
        age: None,
    };
    person_eg.greeting();
    person_eg.how_old();
    sec_person_eg.greeting();
    sec_person_eg.how_old();
}
fn use_hash_map() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    for (key, val) in &map {
        println!("{}'s value is: {}", key, val)
    }
}

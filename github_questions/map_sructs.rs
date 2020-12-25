
I'd like to map an array of structs to an array of there values. How would I do this in Rust?

```lang-rust

pub struct Person {
    name: String
} 

fn main(){
    my_people = vec![Person{name:"Bob".to_string(), Person{name: "Jill".to_string(), Person{name: "Rakim".to_string()}}}]
    // how to map my_people to an array of the names values returing array ["Bob", "Jill", "Rakim"]?
}
```
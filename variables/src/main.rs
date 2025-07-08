// fn main() {
//     let x = 5;
//     println!("the values of x is: {}", x);
//     let x = x + 9;
//     println!("the value of x is: {}", x);
// }

fn main() {
    let x = 27;
    let x = x + 27;

    {
        let x_new = x + 67;
        println!("the fake value of x_new is: {}", x_new)
    }

    println!("the real value of x is: {}", x)
}
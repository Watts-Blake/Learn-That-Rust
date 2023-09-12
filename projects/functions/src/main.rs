fn main() {
    println!("Hello, world!");

fn make_adder( x: i32) -> impl FnMut(i32) -> i32 {
        let mut outer_var = x;
        // move is
        move |y: i32| {
            outer_var+=y;
            // println!("line 46 : {}", outer_var);
            outer_var
        }
    }

    let mut increase_num = make_adder(5);

     println!("{}", increase_num(1));
     println!("{}", increase_num(2));
     println!("{}", increase_num(3));

}

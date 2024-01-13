fn main() {
    /*
     * Rust has tuples, which is always a great way to return multiple values of different types
     * from a single function.
     */
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    /*
     * The line below use pattern matching to destructure the tuple we just created, which is
     * really great that it's supported.
     */
    let (_x, _y, _z) = _tup;

    println!("The value of y is: {_y}");

    // You can also access the tuple items directly:
    let _five_hundred = _tup.0;
    let _six_point_four = _tup.1;
    let _one = _tup.2;

    /*
     * TODO: What's the reason behind having to add underscore to variable names that's showing up
     * as a warning? Looks like it related to some Rust code convention.
     */

    // Arrays in Rust can be type inferenced by simple definition:
    let _a = [1, 2, 3, 4, 5];

    /*
     * Arrays are useful when you want data on the stack rather than the heap, or when you want to
     * make sure that you have a fixed number of elements.
     *
     * Vectors are alternative to arrays that can be dynamically sized.
     *
     * Full type annotation for an array contains it's type and number of elements as shown below:
     */
    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    another_function();

    print_labeled_measurement(5, 'h');

    let added_value = plus_one(1);
    println!("The added value is {added_value}");

    if_in_let_statement();

    //infinite_loop();

    return_values_from_loops();

    condition_while_loop();

    loop_collection();

    loop_collection_for();

    loop_range();
}

// Rust uses snake_case as convention for naming functions
fn another_function() {
    println!("Another function");
}

// In Rust, you must always declare the type of each parameter
// Also note that char type are specified in single quotes
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

// Functions can also specify their return values with the -> syntax
fn plus_one(number: i32) -> i32 {
    return number + 1;
}

// Because if is an expression, it an be used in the right side of a let statement to assign value
// of a variable.
fn if_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

/*fn infinite_loop() {
    loop {
        println!("Again!");
    }
}*/

// You can return a value after the break statement exiting the loop
fn return_values_from_loops() {
    let mut counter = 0;

    let _result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {_result}");
}

fn condition_while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("Liftoff!");
}

fn loop_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn loop_collection_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is {element}");
    }
}

// Range is not inclusive at the end in Rust
fn loop_range() {
    for number in 1..4 {
        println!("{number}!");
    }
}

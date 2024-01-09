fn main() {
    /*
     * Rust has tuples, which is always a great way to return multiple values of different types
     * from a single function.
     */
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    /*
     * The line below use pattern mathcing to destructure the tuple we just created, which is
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
}

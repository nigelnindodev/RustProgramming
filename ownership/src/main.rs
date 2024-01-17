fn main() {
    // In Rust, memory is managed through a system of of ownership, with a set of rules that the
    // compiler checks.
    //
    // It's important to dive a bit more into how Rust uses the stack and heap.
    //
    // The stack and heap are parts of memory available to your code to use at runtime, but they
    // are structured in different ways.
    //
    // The stack stores values in the order it gets them and removes the values in the opposite
    // order. This is last in, first out (LIFO)
    //
    // All the data stored in a stack must have a known, fixed size. Data with an unknown size at
    // compile time or a size that might change must be stored in the heap instead.
    //
    // The heap is less organized. When you put data into the heap, you request a certain amount of
    // space. The memory allocator finds an empty spot in memory that is big enough, marks it as
    // being in use, and returns a pointer to that location.
    //
    // Because the pointer to the heap is a known, fixed size, you can store the pointer on the
    // stack, but when you want the actual data, you must follow the pointer.
    //
    // Pushing to the stack is faster than allocating to the heap because the allocator never has
    // to search for a new place to store new data, the allocation always happens at the top of the
    // stack.
    //
    // Allocating space to the heap also requires more work because the allocator has to find a big
    // enough space to hold the data and then perform bookkeeping to prepare for the next
    // allocation.
    //
    // Accessing data in the heap is slower than fetching from the stack because you have to follow
    // a pointer to get there.
    //
    // Processors are faster if they jump around less in memory. This is because it can do its job
    // better if it works on data that is close to other data (as is on the stack) rather than
    // further away (as it can be on the heap).
    //
    // When you code calls a function, the values passed into the function (including potentially,
    // pointers to data on the heap) and the functions local variables get pushed onto the stack.
    // When the function is over, those values get popped off the stack.
    //
    // Keeping track of what parts of code are using what data on the heap, minimizing the amount
    // of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out
    // of space are all problems that ownership addresses.
    //
    // Once you understand ownership, you will not need to think about the stack and heap very
    // often.
    //
    // OWNERSHIP RULES:
    // - Each value in Rust has an owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.
    //
    string_literal();

    multiple_variables();

    clone_data();

    let _s1 = gives_ownership();

    let s2 = String::from("Hello");

    let _s3 = takes_and_gives_back(s2);

    let (passed_string, len) = calculate_length(String::from("hello"));

    println!("The length of '{}' is {}", passed_string, len);
}

fn string_literal() {
    // Other than immutable string literals, Rust has a String type that manages data allocated on
    // the heap and can store any amount of text unknown at compile time. We can create an instance
    // of it from a string literal as shown below.
    let mut s: String = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    // In the case of a string literal, we know the contents at compile time, so the text is
    // hard coded directly in the final executable.
    //
    // With the String type, in order to support a mutable, growable piece of text, we need to
    // allocate an amount of memory on the heap, unknown at compile time, to hold the contents.
    // This means:
    //
    // - The memory must be requested from the memory allocator at runtime.
    // - We need a way of returning this memory to the allocator when we are done with our String.
    //
    // The first part is done by us, when we call String::from
    //
    // However, the second part is different. In languages with a garbage collector, the GC keeps
    // track of and cleans up memory that isn't being used anymore, and we don't need to think
    // about it.
    //
    // In languages without a GC, it is our responsibility to identify when memory isn't being used
    // anymore an call code to explicitly free it, just as we did to request it. Doing this
    // correctly has historically been a difficult programming problem.
    //
    // Rust takes a different path: the memory is automatically returned once the variable that
    // owns it goes out of scope.
    //
    // When a variable goes out of scope, Rust calls a special function for us. This function is
    // called drop, and it's where the author of String can put the code to return the memory.
    //
    // It may seem simple with the current trivial example, but the behavior of code can be
    // unexpected in more complicated situations when we want to have multiple variables use the
    // data we have allocated on the heap.
}

fn multiple_variables() {
    let x = 5;
    let _y = x;

    // From the above, we now have two variables, both equal to 5. This is indeed what is
    // happening, because integers are simple values with a known, fixed size, and these two 5
    // values are pushed to the stack.

    let s1 = String::from("hello");
    let _s2 = s1;

    // This looks very similar to the previous example, so we might assume that the way it works is
    // the same: that is the second line would make a copy of the value in s1 and bind to s2. But
    // this isn't quite what happens.
    //
    // The actual data is store on the heap, and only metadata for the string is stored in the
    // stack (pointer, length of string, capacity of memory allocated). This is what is copied and
    // not the raw heap data.
    //
    // If Rust also copied the raw data from the heap, that would be an expensive operation.
    //
    // As mentioned earlier, Rust automatically calls the drop function and cleans up heap memory
    // for the variable. Since s1 and s2 both point to the same location the would both try to free
    // the same memory once they go out of scope.
    //
    // This is known as a double free error. Freeing memory twice can lead to memory corruption,
    // which can potentially lead to security vulnerabilities.
    //
    // To ensure memory safety, after the line `let s2 = s1`, Rust considers s1 as no longer valid.
    // Therefore, Rust does not need to free anything when s1 goes out of scope.
    //
    // When you try to use s1 after s2 is created, it won't work and leads to a compilation error.

    //println!("{}, world", s1); // this failed with a borrow of moved value error

    // If you know the terms shallow copy and deep copy from other languages, the concept of
    // copying the pointer, length, and capacity without copying the data probably sounds like
    // making a shallow copy. But because Rust also invalidates the first variable, instead of
    // being called a shallow copy, it's known as a move.
    //
    // With only s2 valid, when it goes out of scope it alone will free up memory.
    //
    // There's a design choice implied by this: Rust will never automatically create "deep" copies
    // of your data. Therefore, automatic copying can be assumed to be inexpensive in terms of
    // runtime performance.
}

fn clone_data() {
    // Should we want to deeply copy the heap data, not just the stack data, we can use a common
    // method called clone.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // This now works fine.
    //
    // When you see a call to clone, you know that some arbitrary code is being executed and that
    // code may be expensive.
}

// The reason that we did not have to call clone for integers is that types such as integers have a
// known size at compile time. These are stored on the stack, so copies of the actual values are
// quick to make. In other words, there is really no difference between deep and shallow copying
// for such cases, so calling clone will not do anything from the usual shallow copying, and we can
// leave it out.
//
// Rust has a special annotation called the `Copy` trait that we can place on types that are stored
// in the stack. If a type implements the `Copy` trait, variables that use it do not move, but
// rather are trivially copied, making them still valid after assignment to another variable.
//
// Rust will not let us annotate a type with `Copy` if the type has in any part implemented the
// `Drop` trait (drop trait should have some function drop() that we have already talked about). If the type needs something to happen when the value goes out of scope and we add
// the `Copy` annotation to that type, we will get a compile time error.
//
// As a general rule, any group of simple scalar values can implement `Copy`, and nothing that
// requires allocation or some form of resource can implement `Copy`.

// gives_ownership function will move it's return value to into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    // In Rust, you can remove the last semi-colon to return the value
    some_string
}

// Function below takes ownership and immediately gives it back
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// While the above two functions work, taking and returning ownership with every function is a bit
// tedious. What if we wanted to let a function use a value and not take ownership? It's also not
// convenient that anything we pass needs to be passed back in order to be used again, in addition
// to any data that results from the body of the function that we might want to return as well.
//
// As an example of the above, Rust does let use return return multiple values from a function
// using a tuple as shown below.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// In the next chapter, we'll explore a Rust feature for using a value without transfer of
// ownership, called `References`

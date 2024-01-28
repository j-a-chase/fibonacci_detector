# Overview

This project is a simple Rust program that can detect via a given file input (sequences.txt) if supplied sequences are valid Fibonacci sequences. For the context of this program, a valid Fibonacci sequence is defined as a sequence longer than two numbers, where each following number is the sum of its two previous numbers.

I wrote this software because I believe that it's a super fun way to apply some of the Rust knowledge I gained as I explored this new language recently. I hadn't gotten quite to the point of using file I/O, but I was able to find resources to help me figure that stuff out and design this program the way I wanted it to be implemented!

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

I developed this program using VSCode as my IDE. The Rust programming language was used for all parts of this project and I did not use any external dependencies.

# Useful Websites

- [Rust By Example (Vectors)](https://doc.rust-lang.org/rust-by-example/std/vec.html)
- [Rust By Example (read_lines)](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html)
- [Educative](https://www.educative.io/answers/what-is-the-stringsplitwhitespace-method-in-rust)
- [Rust Language (appending to a file)](https://doc.rust-lang.org/std/fs/struct.File.html)
- [Reddit (checking for file existence)](https://www.reddit.com/r/rust/comments/14tc0xi/checking_for_file_existence/)
- [Rust Language (removing a file)](https://doc.rust-lang.org/std/fs/fn.remove_file.html)
- [Stack Overflow (checking for numeric characters)](https://stackoverflow.com/questions/51391548/how-to-check-if-string-only-contains-set-of-characters-in-rust)
- [The Rust Programming Language](https://doc.rust-lang.org/book/appendix-02-operators.html)
- [Stack Overflow (removing whitespace characters)](https://doc.rust-lang.org/book/appendix-02-operators.html)

# Future Work

- Add ability for user to give one sequence and get response in the console.
- Improve handling of file input
- Improve file output, so that it's more clear what the results correlate to.
- Change how handling of characters works, so that false value is returned instead of skipping the line.
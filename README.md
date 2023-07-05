# pThis is a simple Rust program that demonstrates string concatenation using the ownership concept.

To run the program, follow these steps:

    Make sure you have Rust installed on your system. If not, you can install it from the official Rust website.

    Open a terminal or command prompt.

    Clone or download the repository to your local machine.

    Navigate to the project directory.

    Build the project by running the command cargo build.

    Run the program with the command cargo run.

The program consists of two functions:

    concatenate_strings: This function takes two string references as input and returns a new String by concatenating them. It uses the push_str method to append the contents of the first string and the second string to the result.

    main: This is the entry point of the program. It initializes two string variables, string1 and string2, with the desired values. It then calls the concatenate_strings function with the references to string1 and string2 as arguments. The result is stored in a variable called concatenated_string. Finally, it prints the concatenated_string to the console.

Feel free to modify the values of string1 and string2 in the main function to see different concatenation results.

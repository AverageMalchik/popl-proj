# popl-proj-grep
Repository for our CS F301 POPL project 

**_IMPLEMENTING GREP FUNCTIONS IN RUST_ **

**<span style="text-decoration:underline;">GROUP NO.12</span>**

  1. MD Jibran - 2020B3A70367G

  2.Vidhan Agarwal - 2020B3A71857G

  3.Bhavya Bansal - 2020B3A71517G

  4.Gautam Jhabak - 20203A71500G


# <span style="text-decoration:underline;">Table Of Contents</span>



1. [Problem Statement](#1problem-statement)

    A.Where is the POPL angle in it?


    B.Has it been solved before?


    C.How is your solution different?

2. Software Architecture

    A.What is the Software Architecture of your solution?


    B.What parts have you reused and what parts have you developed on your own


    C.Where is the testing component placed(local or remote)? Is there a database involved?

3. Popl aspects

    A.What are the Popl Aspects involved in the implementation?


    B. Difficulties Involved.


    C.Thread Safety and locking


    D. Pattern freeing and cleanup

4. How to run the rust code?
1. Results
2. Potential for future work


# 1.Problem Statement


## A.Where is the POPL angle in it?



1. **Ans - 1.Memory Management:**
    *  There is Manual memory management in C (malloc/free).
    * Ownership, borrowing, and lifetimes for memory safety without garbage collection in Rust.
2. **2.Error Handling:**
    * Error codes or return values for error handling is present in C.
    * Result and Option types for structured error handling and optional values in Rust .
3. **3.Concurrency and Safety:**
    * Manual threading and lacks built-in safety guarantees in C.
    * Ownership system ensures safety, provides safe concurrency with constructs like _std::thread_ in Rust.
4. **4.Pattern Matching and Functional Concepts:**
    * Primarily imperative, lacks robust pattern matching in C.
    *  Powerful pattern matching (match), functional features like iterators and closures in Rust .


## B.Has it been solved before?

**Ans**- Yes. Certain developers created a grep functionality for a UNIX Toolset earlier to search for patterns within text files efficiently.


## C.How is your solution different?

**Ans **-  We have tried to implement 2 new functions/commands:-



1. Look for all files in the current directory and in all of its subdirectories in Linux .
2. Search and display the total number of times that a string appears in a file.

2.Software Architecture


## A. What is the software architecture of your soln?

**Ans**- The implementation  are provided for Rust and C.Both C and Rust have their own advantages and disadvantages as specified above.  

 The _grep_ is a powerful command-line utility primarily used in Unix and Unix-like operating systems (such as Linux) to search for specific patterns within files or streams of text.


## B. What parts have you reused and what parts have you developed on your own?

**Ans-**

**REUSABLE CODE LINES**

**[https://github.com/nazmulidris/rust-scratch/tree/main/rust-grep-cli](https://github.com/nazmulidris/rust-scratch/tree/main/rust-grep-cli)**

**1. **Search any line that contains the word in filename on Linux: \
grep 'word' filename

2.Perform a case-insensitive search for the word ‘bar’ in Linux and Unix: \
grep -i 'bar' file1.

**SELF DEVELOPED PARTS**

1.Look for all files in the current directory and in all of its subdirectories in Linux for the word ‘httpd’: \
grep -R 'httpd'** .**

2.Search and display the total number of times that the string ‘nixcraft’ appears in a file named frontpage.md: \
grep -c 'nixcraft' frontpage.md

This is not a client server architecture.


## C.Where is the testing component placed (local or remote)? Is there a database involved?

 Testing Component includes searching of a file in a directory and the files are present in the local system.


No database is invloved as we are searching for files present in the system.


# 3.POPL Aspects


## A.What were the POPL aspects involved in the implementation.



1. 1.File Handling: Utilizing file I/O operations (`File`, `BufRead`, `BufReader`) to read and process data from an external file.
2. 2.Command-Line Arguments: Handling command-line arguments (`env::args()`) to accept input from the user to specify the search pattern and file to search within.
3. 3.Error Handling: Employing Rust's error handling using `Result` and `?` to handle potential errors that may occur during file opening, reading, or other operations (`io::Result`, `File::open`).
4. 4.String Manipulation: Employing string methods (`contains`) for pattern matching and processing text data to find occurrences of a specific pattern within lines read from the file.
5. 5.Iterators and Loops: Utilizing iterators (`iterator`) to sequentially process lines from the file and perform operations on each line.
6. 6.Ownership and Borrowing: Demonstrating Rust's ownership and borrowing system by passing references (`&str`, `&args[..]`) to avoid unnecessary copying of data and manage memory efficiently.
7. 7.Standard Library Usage: Leveraging Rust's standard library (`std::io`, `std::env`) to access common functionalities for I/O operations and command-line argument parsing.
8. 8.Modularity: Organizing code into functions (`main()`) to promote readability, maintainability, and reusability, adhering to the principle of modularity.
9. 9.Error Reporting: Providing user-friendly error messages (`panic!`) to guide users in case of incorrect usage or invalid input.
10. 10.Return Value Handling: Using the `Ok(())` pattern to signify successful execution and returning an empty `Result` on success.


## B.Difficulties encountered

1.We had to understand rust and specifically cargo building and grep such as cargo.toml etc.


## C.Thread Safety and Locking+Pattern freeing and cleanup

It utilizes locking mechanisms (obj_read_lock, grep_attr_lock) to ensure thread safety when accessing shared resources or data structures

Buffer Management:grep_source_clear_data and grep_source_clear functions are responsible for clearing the allocated buffers in the grep_source structure, ensuring proper memory deallocation.

grep_source_clear_data handles different types of grep_source instances, freeing allocated memory accordingly.

Memory Allocation and Deallocation:Functions like grep_source_init_buf, grep_source_init_file, grep_source_init_oid are responsible for initializing the grep_source structure, allocating memory for various fields like buffer, name, path, etc.


 free_filespec is used to deallocate memory associated with a diff_filespec structure.


 FREE_AND_NULL macro is used in grep_source_clear to free allocated memory and set pointers to NULL afterward.


### Object Data Loading and Freeing:


 Functions like grep_source_load_oid and grep_source_load_file are involved in loading object data or file data into memory buffers. They handle memory allocation for the buffer and manage its size.


### General Memory Functions:


 Memory allocation functions like xmallocz, xstrdup_or_null, oiddup, etc., are used for allocating memory with error checking.


##### Standard memory deallocation functions like free and close are used to release allocated memory and close file descriptors.

[https://drive.google.com/file/d/1ig9YzcH7rcwvif7vOk9u2L4dtl0m-7_n/view?usp=sharing](https://drive.google.com/file/d/1ig9YzcH7rcwvif7vOk9u2L4dtl0m-7_n/view?usp=sharing)

[https://drive.google.com/file/d/1TKUNb0Cm-VXIAHy7bxQN4gXorG9YAuCR/view?usp=sharing](https://drive.google.com/file/d/1TKUNb0Cm-VXIAHy7bxQN4gXorG9YAuCR/view?usp=sharing)

[https://drive.google.com/file/d/1y46y8vYo4mY313ZE-JJt27qK0hzR5L4p/view?usp=sharing](https://drive.google.com/file/d/1y46y8vYo4mY313ZE-JJt27qK0hzR5L4p/view?usp=sharing)

[https://drive.google.com/file/d/1MW70yqMJrEBIpxdogS_83gavb67aDSJ3/view?usp=sharing](https://drive.google.com/file/d/1MW70yqMJrEBIpxdogS_83gavb67aDSJ3/view?usp=sharing)

[https://drive.google.com/file/d/1pkpdsqBx32UtPwl0cCpNDpmt8UeuIZmD/view?usp=sharing](https://drive.google.com/file/d/1pkpdsqBx32UtPwl0cCpNDpmt8UeuIZmD/view?usp=sharing)


# 4.How to Run The Rust Code

[https://www.youtube.com/watch?v=T_KrYLW4jw8](https://www.youtube.com/watch?v=T_KrYLW4jw8)


# 6.Potential for future work

A potential future work includes to implement regular expression pattern matching.A C package called PCRE2 (Perl Compatible Regular Expressions 2) offers a number of methods that perform regular expression pattern matching with Perl 5's syntax and semantics. For managing regular expressions in a variety of computer languages, PCRE2 is an effective tool.

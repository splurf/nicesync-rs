# nicesync-rs

## Description
This is a just quick and short demonstration of my understanding of concurrency and conditional thread blocking utilizing the [CondVar](https://doc.rust-lang.org/std/sync/struct.Condvar.html) functionality in the [rust](https://www.rust-lang.org/) programming language.

## Usage
When you run the program it'll continuously read input through *stdin*. Enter `69` to have the word "hehe" written to the terminal. To exit to program, either enter `420` or just leave with `CTRL-C`.

## How it works
When starting the program, before initiating any sort of input through `stdin`, an external thread is initialized which essentially does nothing, consuming no CPU time, until a certain variable is changed. While the thread is blocked, the main thread asks for the user to input shit. If the user inputs a `69` or `420` the variable is modified and then the external thread is *notified* of the modification of the shared variable. Upon notification, the external thread wakes and checks if the shared variable was modified to a particular value. If all goes well, the external thread completes its little sub-task and continues to loop, of course until a `420` is entered where it would then end itself.
# Rust_Final
Eva Gao and Ariel Travers

**Creating multi-threaded word frequency calculator with Rust**

## Folder Organization:
* Cargo.toml file contains the dependecies needed to install crates and configuration.
* Cargo.lock file: keeps track of dependencies and configurations.
* Final_Paper.pdf: pdf version of our paper
* Multithreaded_Word_Counter: an Excel sheet containing our experimental data.
* src folder:
    * main.rs contains our code for the frequency calculator.
    * text folder contains various txt files for testing:
        * mobydict.txt: a full version of the book downloaded from https://gist.github.com/StevenClontz/4445774.
        * example.txt: a file containing words starting with non-alphabetical characters
        * test_1.txt: a text file containing words or phrases separated by newlines.
        * test_2.txt: a text file containing a poem.
        * manymoby.txt: a file containing many copies of mobydict.txt.
        * bible.txt: a King James version of the Bible + mobydict.txt
        * random_readble.txt: a file containing many “lorem ipsum” generated words.
        * random_unreadble.txt: a file containing a portion of random.txt from the Canterbury corps.
        * randomstrnonum.txt: a series of randomly generated strings with the same length. No numbers are included.
        * randomstrnum.txt : a series of randomly generated strings with numbers. All strings have the same length.
        * 3randstrnonum.txt: 3 copies of randomstrnonum.txt
        * 3randstrnum.txt: 3 copies of randomstrnum.txt


## Running the word statistics calculator
* To compile and run our multithread calculator, please cd into the folder src and then enter the command cargo run on the terminal.
* Our main function calls run_tests(), our testing function
    * first converts the filepath to a String in line 31.
    * calls prepare_buff() to load contents into the static string
    * then calls calculate)word_count().
    *  calculate_word() has two parameters. 
        * range0: range of starting letters in u8 for which each thread is responsible. The number of thread spawned can be calculated by the formula 26/range.
        * extras: a boolean indicating extra features. If the boolean is set true, our calculator will include words starting with non-alphabetical characters and also numbers.
    *   Number of threads can be changed by range0.
* analyze_result()will sort the tuple vector in alphabetical order and then prints it.


## Suggested Tests and interpretation
* To run on different text file, just changed the filepath on line 31
* Here are some suggested tests:
    * example.txt illustrates that the extras features work as the file includes some words starting with -
    * test_2.txt illustrates that the punctuation removal is working
    * bible.txt, manymoby.txt, mobydick.txt, and random_readable all are used to show that the calculator works on human readable text
    * randstrnonum,random_unreadble, 3randstrnonum.txt can be run to show that the calculator works on random non-readable strings. 


## Limitations:

* Our word calculator can only work on one single text file at a time. If called on subsequent files, no aggregate word frequency table will be printed.
* Our calculator finds words by splitting by whitespace, so these "words" may be not be valid words. Our accurrancy and meaningfulness depend heavily on the text file.

# Rust_Final
Eva and Ariel

**Creating multi-threaded word frequency calculator with Rust**

## Folder Organization:
* Cargo.toml file contains the dependecies needed to install crates and configuration.
* Cargo.lock file: keeps track of dependencies and configurations.
* src folder:
    * main.rs contains our code for the frequency calculator.
    * text folder contains various txt files for testing:
        * mobydict.txt: a full version of the book downloaded from https://gist.github.com/StevenClontz/4445774.
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


### Running the word statistics calculator
* To compile and run, enter the command cargo run on the terminal.
* Currently, our main function calls run_tests() which loads the random_readable.txt and call calculate_word(). 
* calculate_word() also two parameters. 
    * The first is the range of starting letters each thread is responsible. The number of thread spawned can be calculated by 26/range.
    * a boolean indicating extra features. If the boolean is set true, our calculator will include words starting with non-alphabetical characters and also numbers.
    * currently, our calculator will spawn 4 threads and includes words starting with non-alphaebetical characters.
    * Number of threads can be changed by range.

* To turn off extra features, just pass false as the second parameter.
* To run a different text file, please change the file path parameter in line 31.
* After running, the unique words and their counts will be displayed in the output. The run time is also printed.


#### Limitations:
* Our word calculator can only work on one single text file at a time. If called on .subsequent files, no aggregate word frequency table will be printed.
* Our calculator finds words by splitting by whitespace, so these "words" may be not be valid words. Our accurrancy and meaningfulness depend heavily on the text file.

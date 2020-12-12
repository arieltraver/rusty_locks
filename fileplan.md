# File plan
## *for an extended product

* Each thread initiates and writes to a static vector of tuples.
    * The writing replaces the print function.
    * Results stored in format: word,count
* After all threads are joined, another function analyze\_output is called
    * Sort the vector by key (alphabetical)
    * Store the results in a new file and print as you go

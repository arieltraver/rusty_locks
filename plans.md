# Ideas

## How to break up thread work
* Have each thread work on one portion of the file and save counts to a vector array, combine at the end
* Have hasmaps and every time you enter something in the map, you save its "key" into a key list. Then when preparing the histogram you just look at the keys in the key list
    * All threads share the file and the hashmap
    * Each letter corresponds to a unique "keylist"
    * Each thread looks at words starting with one unique letter or range of letters (depending on how many processors we have)
        * A new value is entered in the keylist when a thread makes a new entry with count = 1 (if the key is there already, then don't add it to the list)
    * The threads write to a shared hashmap, with hash function creating unique values for each word
        * No data races because the threads will never insert the same words
    * At the end, merge threads, then go through keylists and retrieve the count at each value
    * Combined length of all the keylists is the number of unique words

## How to handle large files
* Every page (some number, say 4096 bytes) while the file end hasn't been reached, we load that into the buffer
    * Possible overhead: seek? but also would be for the normal nonthread version
* For each page:
    * Make a new thread and run count(letter, buffer, hashmap, list) for every starting letter
    * Wait for all threads to finish before rewriting the buffer with new data]
    * Rewrite statement happens after the join
        * Maybe put a write lock on the buffer just in case?

## Analysis of runtime cost
* Looking through the whole file: O(filesize)
    * Threads multiply the operation count by number of threads but potentially reduce cost through concurrency
* Copying into the buffer each time: O(filesize)
* Inserting an index into the hashtable: (worst case all unique words, assuming insertion is o(1): O(unique).
    * Updating the hashmap in general is O(filesize).
    * This will be problematic if hashmaps can't be written to concurrently (in which case we would have to make one which can...
* Reading through the list and obtaining the counts: O(unique)

## Analysis of Space Efficiency
* Much better with one single hashtable
    * Still can be large depending on how the String hashmap is implemented in Rust
    * HUGE performance loss if the array overflows the stack (forcing disc reads)
    * However we doubt their implementation would do that...
* The lists won't exceed L(unique) //l is size... there's probably an official way to denote that but haven't learned it

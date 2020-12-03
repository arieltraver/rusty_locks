# Ideas

## How to break up thread work
* Have each thread work on one portion of the file and save counts to a vector array, combine at the end
* Have hasmaps and every time you enter something in the map, you save its "key" into a key list. Then when preparing the histogram you just look at the keys in the key list
    * All threads share the file
    * They each look at words starting with one unique letter or range of letters (depending on how many processors we have)
    * They write to a shared hashmap
        * No collisions because they will never insert the same words
    * When a key is made by finding a new word, that key is saved into a "keylist"
    * At the end, merge threads, then go through keylist and retrieve the count at each value
    * Length of the keylist is the number of unique words

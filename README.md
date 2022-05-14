# bitdb
## TODO 
  * Migrate to CTrie or HAMT based on Ideal Hash Trees paper for in memory store 
  * This implementation can be used to support prefixes in keys by making slight modifications in how the insertion/search process works. Prefixes of keys are important because we can use this for searching by prefix

## Notes 
### CTrie 
Writing a ctrie is not that difficult, we only need to perform CAS operations on hash slots. I do not believe the CAS traversals (for insertion/search) should impact correctness but we should probs yield via futures instead of spinning. 
But the more relevant question could be do we even need to make this concurrent? file IO is needed before we can write to the in memory data structure, file IO is likely to overshadow anything else. 
Perhaps if our persisted data layout was more complex, CTries may make sense, but insertions and reads are a single seek/(read|write) pair.

## Resources 
  * [Riak bitcask paper](https://riak.com/assets/bitcask-intro.pdf)
  * [Ideal Hash Trees](http://lampwww.epfl.ch/papers/idealhashtrees.pdf)

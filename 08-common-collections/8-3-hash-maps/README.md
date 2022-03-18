# 8.3. Storing keys with associated values in hash maps

Type: `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`.

Like vectores, all keys and values must be of the same type respectively. Also like vectors, data is stored on the heap.

How: By using a hashing function that determines how to place keys and values into memory.

Named as hash, map, object, hash table, dictionary, associative array, etc in other programming languages.

## Code samples

[create hash maps](./crates/create_hash_map/src/main.rs)

[hash maps and ownserhip](./crates/hash_maps_and_ownership/src/main.rs)

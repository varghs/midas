# Bitboard Basics
## General Bitboard Techniques
### General Setwise Operations
Essential in testing and manipulating bitboards. Relational test for equality, bitwise boolean perform intrinsic setwise operations, shifting bitboards simulates piece movement, and arithmetical operations are used in bit-twiddling applications and to calculate various hash-indices.
#### Relational
##### Equality
`a == b` - equal
`a != b` - not equal

##### Empty and Universe
```rust
const EMPTY: Bitboard = 0;
const UNIVERSE: Bitboard = 0xffffffffffffffff;
```

#### Bitwise Boolean
##### Intersection
`a & b` - intersection

###### Idempotent
`a & a == a`

###### Commutative
`a & b == b & a`

###### Associative
`(a & b) & c == a & (b & c)`

###### Subset
The intersection of two sets is a subset of them both.
To check if one set is a subset of the other, we compare if the intersection equals the subset.
```rust
fn is_subset(&self, o: Self) -> bool {
    *self & o == *self
}
```

###### Disjoint
Two sets are disjoint if their intersection is 0.
```rust
fn disjoint(&self, o: Self) -> bool {
    *self & o == 0
}
```

##### Union
`a | b` - union

###### Idempotent
`a | a == a`

###### Commutative
`a | b == b | a`

###### Associative
`(a | b) | c == a | (b | c)`

###### Distributive
`a | (b & c) == (a | b) & (a | c)`
`a & (b | c) == (a & b) | (a & c)`

###### Superset
The union of two sets is a superset of both of the sets.

##### Complement Set
`!a`

###### Complement Laws
`a | !a == UNIVERSE`
`a & !a == EMPTY`
`!EMPTY == UNIVERSE`
`!UNIVERSE == EMPTY`

###### DeMorgan's Laws
`!(a | b) == !a & !b`
`!(a & b) == !a | !b`

##### Relative Complement
The set of all things not in a, that are in b. May be interpreted as a bitwise < operation.
`!a & b`

###### Super Minus Sub
Where `-` refers to the symmetrical difference (exclusive or).
`!a & b == (a | b) - a`
`!a & b == b - (a & b)`

##### Implication
Implication may be interpreted as a bitwise <= operation.
`!a | b`

##### Exclusive Or
Implements a bitwise != relation. Also acts as bitwise addition and subtraction (mod 2).
`a ^ b`

###### Commutative
`a ^ b == b ^ a`

###### Associative
(a ^ b) ^ c == a ^ (b ^ c)

###### Distributive
`a & (b ^ c) == (a & b) ^ (a & c)`

###### Own Inverse
`(a ^ b) ^ b == a`

###### Subset
If one operand is a subset of the other, xor (or subtraction) implements the relative complement.

###### Subtraction
While commutative, xor is a better replacement for subtracting from a power of two minus one values.
`(2<sup>n</sup> - 1) - a == a ^ (2<sup>n</sup> - 1)`

##### Equivalence

##### Majority
##### Greater One Sets

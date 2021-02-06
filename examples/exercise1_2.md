# Exercise 1.2.
A _Dyck word_ is a sequence of +1’s and -1’s with the property
that the sum of any prefix of the sequence is never negative. For example,
+1, −1, +1, −1 is a Dyck word, but +1, −1, −1, +1 is not a Dyck word since
the prefix +1 − 1 − 1 < 0. Describe any relationship between Dyck words
and Stack push(x) and pop() operations.

# Solution
Considering push(x) as +1 and pop() as -1 :

A _Dyck word_ of push(x) & pop() operations on a Stack is guaranteed to be
a valid sequence of operations (pop() always yield a value).
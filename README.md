dbxcase
=======

This is an implementation of text case-folding which matches how Dropbox handles file paths.

Dropbox was originally implemented using Python 2.5 (the current version at the time) and used its
`unicode.lower()` function to compare paths case-insensitively. Python 2.5 is long gone, but its
behavior of this function has been preserved to maintain backwards compatibility.

Python 2.5's case-folding is based on Unicode 4.1.0's character database, but does not implement
the case-folding algorithm recommended. Instead, it simply applies the "simple lowercase mapping"
which is a 1:1 character mapping and does not take any context into account. And of course, it
lacks many characters added since 2003.

As a result, it differs in several ways from any modern `to_lowercase()` function like the one
included in the Rust standard library. These differences are important if proper interoperation
with the Dropbox API is desired.

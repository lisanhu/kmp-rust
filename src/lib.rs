/*!
    https://www.geeksforgeeks.org/kmp-algorithm-for-pattern-searching/

    \brief This is a simple KMP algorithm implementation using Rust.

    Given a text txt[0..n-1] and a pattern pat[0..m-1], write a function search(char pat[], char txt[]) that prints all occurrences of pat[] in txt[]. You may assume that n > m.
*/
pub mod kmp;

#[cfg(test)]
mod tests;
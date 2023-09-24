use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Count the number of occurrences of a word in a string
#[pyfunction]
fn count_word_occurrences(haystack: &str, needle: &str) -> PyResult<usize> {
    let mut count = 0;
    for word in haystack.split_whitespace() {
        if word == needle {
            count += 1;
        }
    }
    Ok(count)
}

/// A Python module implemented in Rust.
#[pymodule]
fn python_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(count_word_occurrences, m)?)?;
    Ok(())
}

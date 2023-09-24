# Rust and Python Interoperability with PyO3

https://pyo3.rs/v0.19.2/

## Performance comparison

```Python
text = "Haystack with a needle needle " * 10000000
```

```Python
%timeit count_word_occurrences_py(text, "needle")
4.04 s ± 43.6 ms per loop (mean ± std. dev. of 7 runs, 1 loop each
```

```Python
%timeit count_word_occurrences(text, "needle")
399 ms ± 723 µs per loop (mean ± std. dev. of 7 runs, 1 loop each)
```

Where the Python implementation is
```Python
def count_word_occurrences_py(haystack, needle):
    count = 0                      
    for word in haystack.split(" "):   
        if word == needle:         
            count += 1
    return count
```

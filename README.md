# Performance comparison of various implementations of double linked lists

## Miri

```bash
cargo miri test
```

```txt
test dll::tests::miri ... ok
test dll_raw::tests::miri ... ok
```

## Benchmarks

```bash
cargo bench
```

```terminaloutput
push_only/std_linked_list/1
                        time:   [17.672 ns 17.693 ns 17.713 ns]
                        thrpt:  [56.455 Melem/s 56.521 Melem/s 56.586 Melem/s]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe
push_only/raw_pointer_linked_list/1
                        time:   [17.372 ns 17.551 ns 17.899 ns]
                        thrpt:  [55.870 Melem/s 56.977 Melem/s 57.563 Melem/s]
Found 14 outliers among 100 measurements (14.00%)
  9 (9.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe
push_only/std_linked_list/100
                        time:   [1.6552 µs 1.6573 µs 1.6593 µs]
                        thrpt:  [60.266 Melem/s 60.340 Melem/s 60.415 Melem/s]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
push_only/raw_pointer_linked_list/100
                        time:   [1.6455 µs 1.6591 µs 1.6859 µs]
                        thrpt:  [59.315 Melem/s 60.274 Melem/s 60.772 Melem/s]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe
push_only/std_linked_list/200
                        time:   [3.2975 µs 3.3028 µs 3.3083 µs]
                        thrpt:  [60.454 Melem/s 60.555 Melem/s 60.652 Melem/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
push_only/raw_pointer_linked_list/200
                        time:   [3.2835 µs 3.2896 µs 3.2953 µs]
                        thrpt:  [60.692 Melem/s 60.797 Melem/s 60.910 Melem/s]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
push_only/std_linked_list/500
                        time:   [8.2013 µs 8.2134 µs 8.2259 µs]
                        thrpt:  [60.783 Melem/s 60.876 Melem/s 60.966 Melem/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild
push_only/raw_pointer_linked_list/500
                        time:   [8.1873 µs 8.2001 µs 8.2133 µs]
                        thrpt:  [60.877 Melem/s 60.975 Melem/s 61.071 Melem/s]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
push_only/std_linked_list/1000
                        time:   [17.593 µs 17.632 µs 17.672 µs]
                        thrpt:  [56.588 Melem/s 56.716 Melem/s 56.841 Melem/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
push_only/raw_pointer_linked_list/1000
                        time:   [17.624 µs 17.657 µs 17.689 µs]
                        thrpt:  [56.531 Melem/s 56.635 Melem/s 56.740 Melem/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild

push_then_pop_all/std_linked_list/1
                        time:   [17.392 ns 17.412 ns 17.433 ns]
                        thrpt:  [57.362 Melem/s 57.430 Melem/s 57.499 Melem/s]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
push_then_pop_all/raw_pointer_linked_list/1
                        time:   [17.382 ns 17.403 ns 17.424 ns]
                        thrpt:  [57.392 Melem/s 57.460 Melem/s 57.532 Melem/s]
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) low mild
  7 (7.00%) high mild
  2 (2.00%) high severe
push_then_pop_all/std_linked_list/100
                        time:   [1.6459 µs 1.6494 µs 1.6530 µs]
                        thrpt:  [60.496 Melem/s 60.628 Melem/s 60.758 Melem/s]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) low mild
  1 (1.00%) high severe
push_then_pop_all/raw_pointer_linked_list/100
                        time:   [1.6433 µs 1.6458 µs 1.6484 µs]
                        thrpt:  [60.667 Melem/s 60.762 Melem/s 60.855 Melem/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) low mild
push_then_pop_all/std_linked_list/200
                        time:   [3.2760 µs 3.2882 µs 3.3021 µs]
                        thrpt:  [60.568 Melem/s 60.824 Melem/s 61.049 Melem/s]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
push_then_pop_all/raw_pointer_linked_list/200
                        time:   [3.2945 µs 3.3226 µs 3.3718 µs]
                        thrpt:  [59.316 Melem/s 60.193 Melem/s 60.707 Melem/s]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
push_then_pop_all/std_linked_list/500
                        time:   [8.1667 µs 8.1791 µs 8.1917 µs]
                        thrpt:  [61.038 Melem/s 61.131 Melem/s 61.224 Melem/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
push_then_pop_all/raw_pointer_linked_list/500
                        time:   [8.1925 µs 8.2082 µs 8.2251 µs]
                        thrpt:  [60.789 Melem/s 60.915 Melem/s 61.031 Melem/s]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
push_then_pop_all/std_linked_list/1000
                        time:   [17.647 µs 17.674 µs 17.703 µs]
                        thrpt:  [56.487 Melem/s 56.579 Melem/s 56.668 Melem/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
push_then_pop_all/raw_pointer_linked_list/1000
                        time:   [17.454 µs 17.491 µs 17.529 µs]
                        thrpt:  [57.050 Melem/s 57.172 Melem/s 57.294 Melem/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
```

The tables below summarize the median time per operation size extracted from the benchmark output above.

### Push only (push N elements)

| N     | std::LinkedList | raw_pointer_linked_list |
|-------|-----------------|-------------------------|
| 1     | 17.693 ns       | 17.551 ns               |
| 100   | 1.6573 µs       | 1.6591 µs               |
| 200   | 3.3028 µs       | 3.2896 µs               |
| 500   | 8.2134 µs       | 8.2001 µs               |
| 1,000 | 17.632 µs       | 17.657 µs               |

### Push then pop all (push N, then pop all)

| N     | std::LinkedList | raw_pointer_linked_list |
|-------|-----------------|-------------------------|
| 1     | 17.412 ns       | 17.403 ns               |
| 100   | 1.6494 µs       | 1.6458 µs               |
| 200   | 3.2882 µs       | 3.3226 µs               |
| 500   | 8.1791 µs       | 8.2082 µs               |
| 1,000 | 17.674 µs       | 17.491 µs               |

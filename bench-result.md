# Bench Result

## Bench for jjyr

[code](https://github.com/jjyr/buddy-alloc/blob/master/benches/buddy_alloc.rs)

Benchmarking alloc/16 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 6.3s, enable flat sampling, or reduce sample count to 10.
alloc/16 Bytes          time:   [31.403 ms 32.287 ms 33.644 ms]
thrpt:  [62.334 Melem/s 64.954 Melem/s 66.782 Melem/s]
alloc/32 Bytes          time:   [15.084 ms 15.273 ms 15.587 ms]
thrpt:  [67.274 Melem/s 68.654 Melem/s 69.517 Melem/s]
Found 2 outliers among 20 measurements (10.00%)
1 (5.00%) high mild
1 (5.00%) high severe
alloc/64 Bytes          time:   [8.3002 ms 8.5313 ms 8.9222 ms]
thrpt:  [58.762 Melem/s 61.455 Melem/s 63.166 Melem/s]
Found 4 outliers among 20 measurements (20.00%)
2 (10.00%) high mild
2 (10.00%) high severe
alloc/128 Bytes         time:   [4.4161 ms 4.4767 ms 4.5617 ms]
thrpt:  [57.466 Melem/s 58.558 Melem/s 59.361 Melem/s]
Found 3 outliers among 20 measurements (15.00%)
1 (5.00%) high mild
2 (10.00%) high severe

alloc then free/16 Bytes
time:   [67.635 ms 69.395 ms 71.644 ms]
thrpt:  [29.272 Melem/s 30.221 Melem/s 31.007 Melem/s]
Found 2 outliers among 20 measurements (10.00%)
1 (5.00%) high mild
1 (5.00%) high severe
Benchmarking alloc then free/32 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 8.2s, enable flat sampling, or reduce sample count to 10.
alloc then free/32 Bytes
time:   [32.563 ms 33.060 ms 33.844 ms]
thrpt:  [30.982 Melem/s 31.718 Melem/s 32.201 Melem/s]
Found 4 outliers among 20 measurements (20.00%)
1 (5.00%) high mild
3 (15.00%) high severe
alloc then free/64 Bytes
time:   [20.101 ms 20.378 ms 20.782 ms]
thrpt:  [25.229 Melem/s 25.728 Melem/s 26.083 Melem/s]
Found 1 outliers among 20 measurements (5.00%)
1 (5.00%) high severe
alloc then free/128 Bytes
time:   [11.055 ms 11.129 ms 11.253 ms]
thrpt:  [23.295 Melem/s 23.556 Melem/s 23.712 Melem/s]
Found 2 outliers among 20 measurements (10.00%)
1 (5.00%) high mild
1 (5.00%) high severe

## Bench for Self

[code](./benches/buddy_alloc.rs)

Benchmarking alloc/16 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 9.5s, enable flat sampling, or reduce sample count to 10.
alloc/16 Bytes          time:   [45.147 ms 45.403 ms 45.676 ms]
thrpt:  [45.913 Melem/s 46.189 Melem/s 46.452 Melem/s]
change:
time:   [-2.8480% -2.0436% -1.2538%] (p = 0.00 < 0.05)
thrpt:  [+1.2697% +2.0862% +2.9315%]
Performance has improved.
Benchmarking alloc/32 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 7.0s, enable flat sampling, or reduce sample count to 10.
alloc/32 Bytes          time:   [33.522 ms 33.697 ms 33.901 ms]
thrpt:  [30.931 Melem/s 31.118 Melem/s 31.280 Melem/s]
change:
time:   [-2.2304% -1.5074% -0.7747%] (p = 0.00 < 0.05)
thrpt:  [+0.7808% +1.5305% +2.2813%]
Change within noise threshold.
Found 2 outliers among 20 measurements (10.00%)
1 (5.00%) low mild
1 (5.00%) high mild
Benchmarking alloc/64 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 5.8s, enable flat sampling, or reduce sample count to 10.
alloc/64 Bytes          time:   [27.752 ms 27.912 ms 28.070 ms]
thrpt:  [18.678 Melem/s 18.783 Melem/s 18.892 Melem/s]
change:
time:   [-1.9714% -0.9228% +0.0686%] (p = 0.10 > 0.05)
thrpt:  [-0.0685% +0.9314% +2.0110%]
No change in performance detected.
Found 1 outliers among 20 measurements (5.00%)
1 (5.00%) low mild
Benchmarking alloc/128 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 5.3s, enable flat sampling, or reduce sample count to 10.
alloc/128 Bytes         time:   [25.549 ms 25.680 ms 25.846 ms]
thrpt:  [10.143 Melem/s 10.208 Melem/s 10.260 Melem/s]
change:
time:   [-0.4794% +0.4230% +1.3244%] (p = 0.38 > 0.05)
thrpt:  [-1.3071% -0.4212% +0.4817%]
No change in performance detected.

alloc then free/16 Bytes
time:   [126.45 ms 126.85 ms 127.25 ms]
thrpt:  [16.481 Melem/s 16.533 Melem/s 16.585 Melem/s]
alloc then free/32 Bytes
time:   [74.824 ms 75.123 ms 75.421 ms]
thrpt:  [13.903 Melem/s 13.958 Melem/s 14.014 Melem/s]
alloc then free/64 Bytes
time:   [49.434 ms 49.645 ms 49.833 ms]
thrpt:  [10.521 Melem/s 10.561 Melem/s 10.606 Melem/s]
Found 1 outliers among 20 measurements (5.00%)
1 (5.00%) low mild
Benchmarking alloc then free/128 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 7.7s, enable flat sampling, or reduce sample count to 10.
alloc then free/128 Bytes
time:   [36.371 ms 36.502 ms 36.637 ms]
thrpt:  [7.1552 Melem/s 7.1817 Melem/s 7.2075 Melem/s]
Found 2 outliers among 20 measurements (10.00%)
1 (5.00%) low severe
1 (5.00%) high severe
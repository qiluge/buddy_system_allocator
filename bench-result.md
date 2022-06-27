# Bench Result

jjyr分配慢回收快。

## Bench for jjyr

[code](https://github.com/qiluge/buddy-alloc/blob/master/benches/buddy_alloc.rs)

```
Benchmarking alloc/16 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 9.5s, enable flat sampling, or reduce sample count to 10.
alloc/16 Bytes          time:   [44.658 ms 44.883 ms 45.118 ms]
                        thrpt:  [46.482 Melem/s 46.725 Melem/s 46.960 Melem/s]
                 change:
                        time:   [+36.248% +41.977% +47.064%] (p = 0.00 < 0.05)
                        thrpt:  [-32.002% -29.566% -26.605%]
                        Performance has regressed.
Benchmarking alloc/32 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 7.7s, enable flat sampling, or reduce sample count to 10.
alloc/32 Bytes          time:   [36.677 ms 36.816 ms 36.993 ms]
                        thrpt:  [28.346 Melem/s 28.482 Melem/s 28.590 Melem/s]
                 change:
                        time:   [+92.703% +112.17% +128.26%] (p = 0.00 < 0.05)
                        thrpt:  [-56.189% -52.868% -48.107%]
                        Performance has regressed.
Found 1 outliers among 20 measurements (5.00%)
  1 (5.00%) low mild
Benchmarking alloc/64 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 6.7s, enable flat sampling, or reduce sample count to 10.
alloc/64 Bytes          time:   [31.870 ms 32.051 ms 32.248 ms]
                        thrpt:  [16.258 Melem/s 16.358 Melem/s 16.451 Melem/s]
                 change:
                        time:   [+227.28% +266.47% +301.37%] (p = 0.00 < 0.05)
                        thrpt:  [-75.085% -72.712% -69.445%]
                        Performance has regressed.
Found 2 outliers among 20 measurements (10.00%)
  1 (5.00%) low mild
  1 (5.00%) high mild
Benchmarking alloc/128 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 5.9s, enable flat sampling, or reduce sample count to 10.
alloc/128 Bytes         time:   [28.037 ms 28.187 ms 28.314 ms]
                        thrpt:  [9.2585 Melem/s 9.3002 Melem/s 9.3500 Melem/s]
                 change:
                        time:   [+432.59% +508.89% +578.16%] (p = 0.00 < 0.05)
                        thrpt:  [-85.254% -83.577% -81.224%]
                        Performance has regressed.
Found 2 outliers among 20 measurements (10.00%)
  1 (5.00%) low mild
  1 (5.00%) high mild

alloc then free/16 Bytes
                        time:   [73.489 ms 73.759 ms 74.030 ms]
                        thrpt:  [28.329 Melem/s 28.433 Melem/s 28.537 Melem/s]
                 change:
                        time:   [+34.269% +35.150% +36.011%] (p = 0.00 < 0.05)
                        thrpt:  [-26.476% -26.008% -25.523%]
                        Performance has regressed.
Found 2 outliers among 20 measurements (10.00%)
  1 (5.00%) low mild
  1 (5.00%) high mild
alloc then free/32 Bytes
                        time:   [52.583 ms 52.909 ms 53.238 ms]
                        thrpt:  [19.696 Melem/s 19.818 Melem/s 19.941 Melem/s]
                 change:
                        time:   [+52.106% +61.423% +68.879%] (p = 0.00 < 0.05)
                        thrpt:  [-40.786% -38.051% -34.256%]
                        Performance has regressed.
Benchmarking alloc then free/64 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 8.7s, enable flat sampling, or reduce sample count to 10.
alloc then free/64 Bytes
                        time:   [41.507 ms 41.616 ms 41.729 ms]
                        thrpt:  [12.564 Melem/s 12.598 Melem/s 12.631 Melem/s]
                 change:
                        time:   [+99.183% +108.89% +116.54%] (p = 0.00 < 0.05)
                        thrpt:  [-53.819% -52.128% -49.795%]
                        Performance has regressed.
Found 2 outliers among 20 measurements (10.00%)
  1 (5.00%) low mild
  1 (5.00%) high mild
Benchmarking alloc then free/128 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 7.1s, enable flat sampling, or reduce sample count to 10.
alloc then free/128 Bytes
                        time:   [33.721 ms 33.871 ms 34.088 ms]
                        thrpt:  [7.6901 Melem/s 7.7395 Melem/s 7.7738 Melem/s]
                 change:
                        time:   [+175.42% +190.28% +201.84%] (p = 0.00 < 0.05)
                        thrpt:  [-66.870% -65.550% -63.692%]
                        Performance has regressed.
```

## Bench for Self

[code](./benches/buddy_alloc.rs)

```
Benchmarking alloc/16 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 9.6s, enable flat sampling, or reduce sample count to 10.
alloc/16 Bytes          time:   [45.508 ms 45.803 ms 46.087 ms]
                        thrpt:  [45.504 Melem/s 45.786 Melem/s 46.083 Melem/s]
                 change:
                        time:   [+0.3373% +0.9742% +1.5974%] (p = 0.01 < 0.05)
                        thrpt:  [-1.5723% -0.9648% -0.3362%]
                        Change within noise threshold.
Benchmarking alloc/32 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 7.1s, enable flat sampling, or reduce sample count to 10.
alloc/32 Bytes          time:   [33.709 ms 33.797 ms 33.904 ms]
                        thrpt:  [30.928 Melem/s 31.026 Melem/s 31.106 Melem/s]
                 change:
                        time:   [+0.0917% +0.8351% +1.6320%] (p = 0.04 < 0.05)
                        thrpt:  [-1.6058% -0.8282% -0.0916%]
                        Change within noise threshold.
Found 1 outliers among 20 measurements (5.00%)
  1 (5.00%) high mild
Benchmarking alloc/64 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 5.8s, enable flat sampling, or reduce sample count to 10.
alloc/64 Bytes          time:   [27.814 ms 27.987 ms 28.181 ms]
                        thrpt:  [18.605 Melem/s 18.733 Melem/s 18.850 Melem/s]
                 change:
                        time:   [-0.6439% +0.3711% +1.4974%] (p = 0.55 > 0.05)
                        thrpt:  [-1.4753% -0.3697% +0.6481%]
                        No change in performance detected.
Found 1 outliers among 20 measurements (5.00%)
  1 (5.00%) high severe
Benchmarking alloc/128 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 5.3s, enable flat sampling, or reduce sample count to 10.
alloc/128 Bytes         time:   [25.351 ms 25.418 ms 25.484 ms]
                        thrpt:  [10.287 Melem/s 10.313 Melem/s 10.341 Melem/s]
                 change:
                        time:   [-1.8891% -1.0585% -0.2163%] (p = 0.02 < 0.05)
                        thrpt:  [+0.2168% +1.0698% +1.9255%]
                        Change within noise threshold.
Found 1 outliers among 20 measurements (5.00%)
  1 (5.00%) high mild

alloc then free/16 Bytes
                        time:   [126.83 ms 127.28 ms 127.70 ms]
                        thrpt:  [16.423 Melem/s 16.477 Melem/s 16.536 Melem/s]
                 change:
                        time:   [-0.1337% +0.3412% +0.7874%] (p = 0.17 > 0.05)
                        thrpt:  [-0.7813% -0.3401% +0.1338%]
                        No change in performance detected.
Found 1 outliers among 20 measurements (5.00%)
  1 (5.00%) low mild
alloc then free/32 Bytes
                        time:   [75.044 ms 75.283 ms 75.535 ms]
                        thrpt:  [13.882 Melem/s 13.928 Melem/s 13.973 Melem/s]
                 change:
                        time:   [-0.3085% +0.2137% +0.7586%] (p = 0.43 > 0.05)
                        thrpt:  [-0.7529% -0.2132% +0.3095%]
                        No change in performance detected.
Found 1 outliers among 20 measurements (5.00%)
  1 (5.00%) high mild
alloc then free/64 Bytes
                        time:   [49.115 ms 49.291 ms 49.493 ms]
                        thrpt:  [10.593 Melem/s 10.637 Melem/s 10.675 Melem/s]
                 change:
                        time:   [-1.2411% -0.7137% -0.1407%] (p = 0.02 < 0.05)
                        thrpt:  [+0.1409% +0.7189% +1.2567%]
                        Change within noise threshold.
Found 1 outliers among 20 measurements (5.00%)
  1 (5.00%) high severe
Benchmarking alloc then free/128 Bytes: Warming up for 3.0000 s
Warning: Unable to complete 20 samples in 5.0s. You may wish to increase target time to 7.7s, enable flat sampling, or reduce sample count to 10.
alloc then free/128 Bytes
                        time:   [36.494 ms 36.593 ms 36.691 ms]
                        thrpt:  [7.1446 Melem/s 7.1638 Melem/s 7.1832 Melem/s]
                 change:
                        time:   [-0.4349% +0.3548% +1.1335%] (p = 0.41 > 0.05)
                        thrpt:  [-1.1208% -0.3536% +0.4368%]
                        No change in performance detected.
```
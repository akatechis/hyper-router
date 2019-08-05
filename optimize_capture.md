# Journal

This document is a little journal I'll be using to take notes as I try to
optimize parameter capture for the router, to try and squeeze a bit more
performance out of it.

## Setup
Before I did any work, I wrote a few benchmarks for some of the scenarios I'd
like to optimize: capturing many parameters, capturing a single parameter, and
matching against a static route.
The last one is not expected to be a costly operation, but I added it for
completeness. I created a branch, ran the benchmarks and got the following
results:

```
capture many parameters time:   [6.7795 us 6.8782 us 6.9965 us]                                     
                        change: [-2.6178% -0.1289% +2.1270%] (p = 0.92 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

capture single parameter                                                                             
                        time:   [1.4063 us 1.4227 us 1.4409 us]
                        change: [-6.5114% -4.0825% -1.8122%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

match static route      time:   [927.36 ns 938.40 ns 951.05 ns]                                
                        change: [-2.6042% -0.1025% +2.4201%] (p = 0.93 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild

```


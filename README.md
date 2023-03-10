# aoc22

Advent of code solutions 2022. This repository contains two sets of solutions:
one written in Python, and one written in Rust.

The Python solutions are the ones I initially used to solve the puzzles, and (with
the exception of day 17 part 2), they were all written on the day of the puzzle
release. The Python solutions are fully automated - they will fetch the input data
and submit a solution automatically.

To run the Python solutions, install the dependencies from `aoc22_py/requirements.txt`,
then run `python -m aoc22_py all`.

The Rust solutions were all written after the end of Advent of Code, with the aim
of bringing the total execution time as low as possible. They also come with an
automated runner, which tests the solutions against my puzzle input data and
solutions, and measures the execution time.

To run the Rust solutions, go to `aoc22_rs` and run `cargo run --release`.

## Scores

Puzzles release at 5am in my time, so I often don't get to them until later.

![Screenshot of scores: see below for plain text.](other/scores.png)

<details>
<summary>Plain text</summary>

> You have 233 points.
>
> ```
>       --------Part 1--------   --------Part 2--------
> Day       Time   Rank  Score       Time   Rank  Score
>  25   02:32:36   4077      0   09:09:30   5850      0
>  24   00:33:41    342      0   00:52:25    642      0
>  23   07:52:36   7024      0   07:55:57   6755      0
>  22   01:04:14   1675      0   22:36:52   7122      0
>  21   09:47:42  13073      0   10:28:06  10153      0
>  20   14:36:13  11051      0   18:35:56  11760      0
>  19   20:12:05   8473      0   20:15:36   7347      0
>  18   04:55:24   8865      0   05:24:14   6246      0
>  17   11:32:33   9886      0       >24h  16786      0
>  16   09:07:22   6996      0   10:34:38   4202      0
>  15   18:03:13  26015      0   18:55:04  20308      0
>  14   03:07:47   9698      0   04:40:06  11310      0
>  13   08:36:40  17840      0   08:42:58  16738      0
>  12   00:15:13    598      0   00:19:19    644      0
>  11   06:54:28  23085      0   07:54:05  17881      0
>  10   00:26:08   5561      0   00:46:30   4695      0
>   9   14:59:51  46295      0   15:10:31  36621      0
>   8   00:04:14     77     24   00:12:37    212      0
>   7   00:16:25    394      0   00:22:20    510      0
>   6   00:01:44     76     25   00:02:10     62     39
>   5   00:08:54    324      0   00:13:36    661      0
>   4   00:02:49    279      0   00:30:41   9748      0
>   3   00:02:18     34     67   00:04:07     32     69
>   2   00:03:39     92      9   00:08:35    443      0
>   1   00:03:26   1458      0   00:04:17    899      0
> ```
</details>

## Rust timings

Timings are measured in release mode on my laptop, a ThinkPad T460 with an i7-6600U CPU and 16GB RAM.

![Screenshot of timings: see below for plain text.](other/rust_timings.png)

<details>
<summary>Plain text</summary>

| Day   | Parse     | Part 1        | Part 2       | Total        |
|:-----:|:---------:|:-------------:|:------------:|:------------:|
| 1     | 56.023??s  | 290ns         | 477ns        | 56.79??s      |
| 2     | 187.456??s | 2.954??s       | 2.976??s      | 193.386??s    |
| 3     | 124.967??s | 27.697??s      | 39.923??s     | 192.587??s    |
| 4     | 67.185??s  | 1.447??s       | 1.429??s      | 70.061??s     |
| 5     | 52.261??s  | 16.316??s      | 15.563??s     | 84.14??s      |
| 6     | 1ns       | 1.962??s       | 2.935??s      | 4.898??s      |
| 7     | 35.248??s  | 139ns         | 109ns        | 35.496??s     |
| 8     | 30.478??s  | 423.562??s     | 523.804??s    | 977.844??s    |
| 9     | 215.053??s | 855.782??s     | 867.558??s    | 1.938393ms   |
| 10    | 5.025??s   | 45ns          | 8.194??s      | 13.264??s     |
| 11    | 3.665??s   | 18.448??s      | 12.322681ms  | 12.344794ms  |
| 12    | 36.945??s  | 184.705??s     | 183.824??s    | 405.474??s    |
| 13    | 483.762??s | 4.227??s       | 18.061??s     | 506.05??s     |
| 14    | 92.72??s   | 337.52??s      | 3.054856ms   | 3.485096ms   |
| 15    | 9.38??s    | 650ns         | 161.779??s    | 171.809??s    |
| 16    | 113.615??s | 4.976317ms    | 2.210594089s | 2.215684021s |
| 17    | 47.83??s   | 408.817??s     | 4.060002ms   | 4.516649ms   |
| 18    | 178.263??s | 427.75??s      | 65.158987ms  | 65.765ms     |
| 19    | 25.296??s  | 395.008161ms  | 1.866647659s | 2.261681116s |
| 20    | 153.772??s | 76.965818ms   | 1.212645813s | 1.289765403s |
| 21    | 646.944??s | 16.299??s      | 68.354??s     | 731.597??s    |
| 22    | 206.087??s | 83.789??s      | 196.987??s    | 486.863??s    |
| 23    | 73.902??s  | 3.326808ms    | 367.199986ms | 370.600696ms |
| 24    | 59??s      | 20.134742ms   | 66.200979ms  | 86.394721ms  |
| 25    | 10.575??s  | 253ns         | 0ns          | 10.828??s     |
|       |           |               |              |              |
| Total | 2.915453ms| 503.224498ms  | 5.809977025s | 6.316116976s |

</details>

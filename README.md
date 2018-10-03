# rlstats-cli

**RocketLeagueStats has been shut down. [See more here](https://rocketleaguestats.com/)**

A Rust CLI program for accessing data from [Rocket League Stats](https://rocketleaguestats.com/).

A small sample of the kind of data that can be retrieved:

```
> rlstats search Cldfire --select 0

Display Name  mem::forget(cldfire);
UniqueID      76561198174976054
Platform      Steam
Profile URL   https://rocketleaguestats.com/profile/Steam/76561198174976054

Requested    Created      Updated      Next Update
Oct 11 2017  Jun  6 2016  Oct 15 2017  Oct 15 2017

Statistics
Wins        3139
Goals       8532
MVPs        1599
Saves       4778
Shots       17874
Assists     3584

Ranked
Season  Playlist              Points  Played  Tier  Division
2       Ranked Duel           597     88      6     3
        Ranked Doubles        828     1094    9     0
        Ranked Solo Standard  523     172     5     4
        Ranked Standard       721     689     8     0
3       Ranked Duel           709     43      9     2
        Ranked Doubles        804     1303    10    2
        Ranked Solo Standard  616     17      8     2
        Ranked Standard       785     606     10    2
4       Ranked Duel           743     17      9     1
        Ranked Doubles        830     333     10    1
        Ranked Solo Standard  493     5       0     0
        Ranked Standard       983     492     12    2
5       Ranked Duel           687     7       0     0
        Ranked Doubles        818     7       0     0
        Ranked Solo Standard  493     5       0     0
        Ranked Standard       997     7       0     0
6       Ranked Duel           687     0       0     0
        Ranked Doubles        818     0       0     0
        Ranked Solo Standard  493     0       0     0
        Ranked Standard       997     0       0     0
```

For Rust programmatic access, see [rlstats-rs](https://github.com/Scetch/rlstats-rs).

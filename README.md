# Invaders

Invaders is an open source terminal arcade game with audio, based off of the "Space Invaders" classic arcade game.

This game was initially developed for a presentation at [OSCON Open Source Software Superstream Series: Live Coding—Go, Rust, and Python](https://learning.oreilly.com/live-training/courses/oscon-open-source-software-superstream-series-live-codinggo-rust-and-python/0636920410188/) and then adapted for inclusion as an example project for the 2020 version of [Ultimate Rust Crash Course](https://www.udemy.com/course/ultimate-rust-crash-course/?referralCode=AF30FAD8C6CCCC2C94F0).  The tags `part-1`, `part-2`, etc. correspond to the various stages of the original presentation.

Since the original presentations, folks continue to tinker and improve the game. Feel free to fork this repository, make a change, and submit a pull request if you have a good idea!

### Sound Files

If you want the sound files used in the course video, but don't want to clone the repository, here are all the sounds in two different archive formats (the sound files are the same):

- [sounds.zip](https://github.com/CleanCut/invaders/files/6312508/sounds.zip)
- [sounds.tar.gz](https://github.com/CleanCut/invaders/files/6312511/sounds.tar.gz)

Notice the new path to the sound files used in the course.
It is now under `audio` > `original`.
All new contributions can be placed under `audio` > `contributions`.

We have now a new set of instrumental music sound files in MP3 format.
You may listen to the sounds on [Musescore.com](https://musescore.com/user/9047536/sets/5156900).

### Dependencies on Linux

Audio should work out-of-the-box on macOS, Windows, and iOS.  For Linux, the
downstream package for actually _playing_ sound ([CPAL]) requires
the *Alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev pkg-config
```

## Community Games!

Were you inspired to make your own terminal-based game? Open a PR to add it to the list here!

* [Pong](https://github.com/basilkohler/rusty_pong)
* [TETRIS](https://github.com/madchicken/rust-tetris)
* [Columns](https://github.com/Rendez/rust_columns)

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).

## Sponsor

Star this repository and [sponsor](https://github.com/sponsors/CleanCut) me to support projects like this. 💖

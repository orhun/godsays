![terry](https://user-images.githubusercontent.com/24392180/122981344-50e0bb00-d3a2-11eb-9132-2c0b4733457f.jpg)

_RIP [Terry A. Davis](https://en.wikipedia.org/wiki/Terry_A._Davis) 1969-2018_

# god says

Rust port of the programmer Terry Davis' `"god says"` (AKA `GodSpeaks`) program.

---

Terrence Andrew Davis (December 15, 1969 – August 11, 2018) was an American programmer who created and designed the operating system [`TempleOS`](https://en.wikipedia.org/wiki/TempleOS) alone. It was a highly complex and unusual undertaking for one person, as the project extended to building core components such as programming language, editor, compiler and kernel virtually from scratch. Davis also posted video blogs to social media, and by the time of his death, had amassed a small online following. He often referred to himself as "the smartest programmer that's ever lived". _From_ [Wikipedia](https://en.wikipedia.org/wiki/Terry_A._Davis).

About TempleOS, from his own words:

> TempleOS is God's official temple.  Just like Solomon's temple, this is a
community focal point where offerings are made and God's oracle is consulted.

Contained in Terry's TempleOS masterpiece were various random text generators. Terry believed that by generating this text one could "speak to God". Through making random associations in the text, much like a [Rorschach ink-blot test](https://en.wikipedia.org/wiki/Rorschach_test).

For that purpose, he designed a random number generator called `god`, and used it in TempleOS to generate text.

The original source of this program can be found [here](https://github.com/cia-foundation/TempleOS/tree/archive/Adam/God).

Besides the original program which was written in `HolyC`, he was using a bash script to generate text:

```sh
#!/bin/bash
# This prints random words.
echo "$(gshuf -n 32 ./Happy.TXT --random-source=/dev/urandom | tr '\n' ' ')"
```

So I reproduced this logic in Rust and created a simple CLI program ([godsays](#cli)) and a webserver ([godsays-server](#server)) for extended use-cases (and tribute to TAD).

> Please note that some of the words may be offensive - I am merely using Terry's original wordlist and thus cannot be held responsible for any of the views expounded by God in the generated text. The only modification that has been made to Terry's text is the replacement of underscores with spaces.

## Installation

## Usage

### CLI

`godsays` command line tool simply selects 32 random words from [Happy.TXT](./Happy.TXT) and prints them out.

```sh
cargo run
```

output:

```
sloth special case I'll ask nicely incredibly in a galaxy far far away what do you want I'm done slumin oh no the enquirer really by the way that's for me to know Isn't that special don't mention it baffling furious I'll think about it Han shot first downer unsung hero super computer horrendous ahh who's to say You da man I give up cosmetics it'd take a miracle stuff holy grail I'll be back
```

\* Use of the `--release` flag embeds the [Happy.TXT](./Happy.TXT) into the binary.

#### Docker

Pull the latest image from Docker Hub and run the container:

```sh
docker pull orhunp/godsays
docker run -t orhunp/godsays
```

Or do it manually:

```sh
docker build -f docker/cli/Dockerfile -t godsays .
docker run -t godsays
```

### Server

`godsays-server` is a simple HTTP server with the same purpose of `godsays`. It returns the randomly generated text on a `GET /` request. (Use `GET /json` for JSON output)

```sh
curl https://godsays.xyz
```

output:

```
ba ha you know a better God do not disturb its trivial obviously ho ho ho failure to communicate if anything can go wrong do you like it in a perfect world that's all folks eh Mission Accomplished super computer Trump kick back vice liberal gosh baffling I'm in suspense holier than thou frown in a galaxy far far away in practice China tree hugger scum snap out of it I'm on a roll joyful money what's it to you
```

to run locally:

```sh
cargo run --features server --bin godsays-server
```

\* A fun use case is creating a git alias as follows:

```sh
git config --global alias.godsays '!git commit -m "$(curl -s https://godsays.xyz)"'
```

#### Docker

Pull the latest image from Docker Hub and run the container:

```sh
docker pull orhunp/godsays-server
```

Or do it manually:

```sh
docker build -f docker/server/Dockerfile -t godsays-server .
docker run --rm -e "ADDR=0.0.0.0:3000" -dp 3030:3000 godsays-server
```

## See also

* https://templeos.org/
* [TempleOS Archive](https://archive.org/details/TerryADavis_TempleOS_Archive)
* https://www.youtube.com/watch?v=xDHQ1yBbS-Q (Tribute Terry Davis)
* https://www.youtube.com/watch?v=mBgIBF9Y6PE (TempleOS: Terry Responds to the Haters)
* https://jcpsimmons.github.io/Godspeak-Generator/ (Javascript port)
* https://github.com/rethyxyz/godspeaks (Python port)
* https://christine.website/blog/templeos-2-god-the-rng-2019-05-30 (`god`, the Random Number Generator)

## License

[The MIT License](https://opensource.org/licenses/MIT)

## Copyright

Copyright © 2021, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)

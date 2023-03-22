# kbd2csv

## Introduction
You are doing osdev and wrote your nice PS/2 or USB keyboard driver, now you have a bunch of keycodes.
Unfortunately, implementing keyboard layouts in a hobby OS is generally very tedious and results in ugly spaghetti code. Not only that, but most osdevers will only hardcode one keymap (or even part of it!).
It's very easy to end up with something like [this (my own code)](https://github.com/the-strife-project/keyboard/blob/master/src/layout.hpp).
There are many examples of this, on pretty much all independent OS: like [this one](https://github.com/managarm/managarm/blob/cf4176fc3e02dc19b65c16e45a6d81099c2fbb68/drivers/kbd/src/main.cpp#L554), [this one](https://github.com/vlang/vinix/blob/feada1ca3dccca98e953f0646b350688b8468abf/kernel/modules/dev/console/console.v#L64), [this one](https://github.com/cia-foundation/TempleOS/blob/c26482bb6ad3f80106d28504ec5db3c6a360732c/Kernel/SerialDev/Keyboard.HC#L2), and [this other one](https://github.com/skift-org/skift/blob/7bc22b9e7924a7d185088cc4b39ef37b8831a378/src/libs/karm-events/keys.inc).

In order to avoid this in [my new OS](https://github.com/Daisogen), I spent an afternoon looking at keymap-related projects.
One of them, [kbd](https://kbd-project.org/), implements [relatively simple layout files](https://github.com/legionus/kbd/blob/master/data/keymaps/i386/qwerty/us.map); however, they're still hard to parse, and keymap files form a hierarchy.

`kbd2csv` converts this format into very simple to parse CSV files in the lines of:

```
[keycode],[no modifiers],[shift],[altgr],[left ctrl],[right ctrl]
```

Such as:

```
1,Escape,,,,
2,one,exclam,,,
3,two,at,at,,
4,three,numbersign,,,
5,four,dollar,dollar,Control_backslash,
6,five,percent,,,
7,six,asciicircum,,,
8,seven,ampersand,braceleft,Control_underscore,
9,eight,asterisk,bracketleft,Delete,
10,nine,parenleft,bracketright,,
11,zero,parenright,braceright,,
```

While it does not convey the full functionality of `kbd`, it's good enough for independent projects, and turns out to be very low-effort high-reward.

## Usage
Clone the official kbd repository and execute `kbd2csv` with the layout name to generate the full self-contained CSV layout file:
```bash
$ git clone https://github.com/legionus/kbd
$ git clone https://github.com/jlxip/kbd2csv
$ cd kbd2csv && cargo build --release
$ target/release/kbd2csv ../kbd/data/keymaps/i386/qwerty/us.map us.csv
```

Pretty please link to this project if you end up using its files.

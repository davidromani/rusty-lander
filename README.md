Rusty Lander v0.8.4
===================

A Jupiter Lander video game remake made with Rust & Bevy ECS game engine.

---

### Installation requirements

* Git ≥ v2.0
* Rust ≥ v1.8
* Cargo ≥ v1.80

### Installation instructions

```bash
$ git clone git@github.com:davidromani/rusty-lander.git
$ cd rusty-lander
$ cargo run --release
```

### Game instructions

#### Goal

Land on one of three platforms. Vertical velocity must come within the yellow area of the scale. The score, according to
the velocity, is multiplied by the number under platform. You'll be refueled on a successful landing. Every landing the
gravity increases.

#### Controls

* press <kbd>A</kbd> or arrow <kbd>LEFT</kbd> to push spaceship right
* press <kbd>D</kbd> or arrow <kbd>RIGHT</kbd> to push spaceship left
* press <kbd>2</kbd> or <kbd>SPACE</kbd> to enable a big thrust up
* press <kbd>W</kbd> or arrow <kbd>UP</kbd> to enable a medium thrust up
* press <kbd>S</kbd> or arrow <kbd>DOWN</kbd> to enable a small thrust up

### References

Read about the original Jupiter Lander video game article in
the [Wikipedia](https://en.wikipedia.org/wiki/Jupiter_Lander).

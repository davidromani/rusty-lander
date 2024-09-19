Rusty Lander v0.3
=================

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
$ cargo run
```

### Game instructions

#### Goal

Land on one of three platforms. Vertical velocity must come within the yellow area of the scale. The score, according to
the velocity, is multiplied by the number under platform. You'll be refueled on a successful landing. Every landing the
gravity increases.

#### Controls

* press key `A` or arrow `LEFT` to push spaceship right
* press key `D` or arrow `RIGHT` to push spaceship left
* press key `2` to enable a big thrust up
* press key `W` to enable a medium thrust up
* press key `S` or `SPACE` to enable a small thrust up

### References

Read about the original Jupiter Lander video game article in
the [Wikipedia](https://en.wikipedia.org/wiki/Jupiter_Lander).

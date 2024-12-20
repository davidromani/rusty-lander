Changelog
=========

##### Version 0.08.005 (WIP)

* keep working
* fix playing music overlap on spaceship just landed bug
* persist best score value so far
* cargo dependencies update

##### Version 0.08.004 (2024-10-26)

* add music

##### Version 0.08.003 (2024-10-20)

* enable Linux & Mac builds

##### Version 0.08.002 (2024-10-20)

* add missing Trunk.toml config

##### Version 0.08.001 (2024-10-20)

* add missing web resources for Trunk

##### Version 0.08.000 (2024-10-20)

* replace rusty_planet menu background image
* add CI/CD workflow

##### Version 0.07.000 (2024-10-10)

* fix un-paused sfx on ship landing problem
* spawn intro music audio bundle on menu
* improve ship_air_scape sfx loop effect
* add CREDITS.md

##### Version 0.06.000 (2024-10-09)

* fix "Out of fuel" message
* better despawn "Out of fuel" info panel behaviour
* spawn explosion sound effect on spaceship crashed
* manage air scape and thruster sound effects on spaceship movement
* add rust to spaceship sprite
* improve landscape image
* add pixel art & sound FX credits

##### Version 0.05.000 (2024-09-29)

* increase gravity after each successful landing
* show out of fuel message
* fix speedometer mark value after landed or crashed

##### Version 0.04.000 (2024-09-22)

* add game menu instructions section
* fix problem with hidden score text after update
* fix bad scoring calculation after landed
* fix double crash problem
* fix problem during update_scoring_text_system execution, score always updated after the following landing
* add more difficult, divide by 5 refuel quantity
* add WorldBoundsVertices resource from SVG
* better aspect ratio window fit
* better background image

##### Version 0.03.000 (2024-09-21)

* add README game instructions
* improve keyboard control keys
* render more accurate yellow speedometer bar
* better computed score after landing
* add rusty planet main menu screen
* better state scoped spawning systems
* add background UI black bars

##### Version 0.02.000 (2024-09-19)

* cargo dependencies update
* make window not resizable
* refactor crash collisions system
* refactor color schema
* replace default font by VT323
* set spaceship friction to zero
* change default initial gravity value
* add dynamic thrust particles property effect behaviour
* show fuel, score & high score UI texts
* update score after spaceship landing
* add space key pressed after landing system

##### Version 0.01.000 (2024-09-12)

* first POC approach

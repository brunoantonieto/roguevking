# Roguevking
A rogue-like game writen in Rust.

## Roadmap to alpha
Basic stuff should be working in order to have a mvp, by priority:
- Collision Handler
must fix the collision handler, there's a bug where the pngs do not exactly matches the collision rects
- Sprites
define the size of the assets and develop the logic for movement animation
- Movement
improve cameras, develop dash (?)
- Combat
improve combat systems and define hps, damage etc
  - Enemies
  - Attack Animation
  - Weapons
- Stages
develop the logic of the stages, the idea is:
  - stage-world model
  - starts on 1-1 until 10-10 (alpha until 1-10)
  - bosses with legendary items on 10
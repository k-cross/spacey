# Enemy Design Ideas

Brainstorm doc for expanding enemies beyond "move straight down." Not decided —
notes to think over. The core question I'm weighing:

> **Movement patterns vs. AI mechanics** — do enemies get more interesting by
> *how they move* (scripted paths, weaves, formations) or by *how they decide*
> (aiming, reacting to the player, adaptive behavior)?

See the framing section at the bottom for the trade-off.

## Where the code stands today

- `EntityType`: `Player`, `Enemy`, `Laser`, `Explosion`, `Asteroid` (`game.rs`).
- An enemy is just `Velocity { dx: 0.0, dy: 0.02 }` + a collider. Every enemy is
  identical — there's no field distinguishing one from another.
- Lasers only travel **up** and are implicitly player-owned in the collision loop.
  Nothing travels down and hurts the player yet.
- Player already has `shield` (3) + `invincibility_timer`, so taking hits is modeled.

Two gaps to fill before "attack moves" exist at all:
1. **Enemy projectiles** — no downward hostile shots exist.
2. **Per-enemy behavior/state** — needed for anything non-uniform.

## Infrastructure choices (pick before types/patterns)

**Distinguishing enemy types** (matches the SoA / parallel-array style):
- Add an `EnemyKind` enum + `Vec<Option<EnemyKind>>` array. Clean, mirrors the
  existing `asteroids: Vec<Option<AsteroidData>>`.
- Or a fuller `Vec<Option<EnemyData>>` struct: `kind`, `fire_cooldown`, `health`,
  `pattern_phase: f32`. Better once behavior gets stateful. **Most patterns below
  need some per-enemy state**, so this is the choice that unblocks the most ideas.

**Enemy shots:**
- New `EntityType::EnemyLaser` — smallest change; a separate collision pass vs. the
  player. (Collision loop already separates enemies vs. lasers.)
- Or an `owner`/`friendly: bool` flag on lasers so one type serves both.

## Enemy type ideas

| Type | Behavior | Teaches the player |
|------|----------|--------------------|
| **Grunt** (current) | Straight down, no shots | Baseline |
| **Shooter** | Slower descent, fires down-aimed shot on cooldown | Watch for projectiles |
| **Sniper/Aimer** | Fires toward player's current position (`dir = player - self`, normalized) | Keep moving |
| **Weaver** | Sine-wave horizontal drift while descending | Timing dodges |
| **Kamikaze** | Locks onto player X, accelerates down | Bait-and-juke |
| **Turret/Mine** | Parks at a Y line, sprays radial shots | Area denial |
| **Tank** | Multi-hit health, slow — needs `health` field | Sustained fire |
| **Splitter** | On death spawns 2–3 grunts (spawn plumbing already exists) | Positioning |

## Attack / shot pattern ideas
- **Single aimed shot** — cheapest, high impact.
- **Spread (3-way)** — three shots at ±15°.
- **Radial burst** — N shots evenly around a circle (pairs with turret).
- **Volley** — 3 shots in quick succession via a burst counter in `EnemyData`.

## Movement pattern ideas
- **Sine weave** (needs `phase` state).
- **Swoop-in** — descend, pause at a Y line, then dive.
- **Formation spawns** — spawn a row / V of grunts at once (`spawn_wave()` helper).
- **Edge strafers** — enter from left/right instead of top (currently top-only).

## The decision: movement patterns vs. AI mechanics

- **Movement patterns** (weave, swoop, formations, edge strafers): deterministic,
  easy to test, "bullet-hell / arcade" feel. Cheap to add — mostly math on
  `position`/`velocity` plus a `phase` field. Lower risk, very tunable.
- **AI mechanics** (aiming, kamikaze lock-on, reacting to player): more dynamic and
  replayable, but harder to tune and test, and can feel unfair without
  telegraphing. Requires reading player state each tick.
- They aren't exclusive — an aimed **Shooter** is a small dose of AI; a **Weaver**
  is pure movement. A middle path: mostly scripted movement + *one* reactive touch
  (aimed shots) for spice.

## Suggested first slice (whenever ready)

One new type — the **Shooter** — plus the enemy-projectile plumbing. Small,
self-contained, and it proves out the `EnemyData` + `EnemyLaser` infrastructure
that nearly everything else reuses.

> **Note:** enemy projectiles are the single biggest gameplay change — they turn a
> dodge-and-shoot game into a bullet-dodger. Everything else is a variation once
> that exists. Touches `game.rs` (logic) and `game_ui.rs` (rendering).

## Weapon types & upgrades

Where the player weapon stands today: `fire_laser()` spawns one laser straight up
(`dy: -0.05`, collider `0.02`) on an 8-frame cooldown (`frame > last_fire_frame + 8`).
No power-ups exist. To make weapons swappable, the cheapest model is a
`current_weapon: Weapon` field on `GameState` that `fire_laser()` reads to decide
count/spread/cooldown — no per-laser type needed for most of these.

### Weapon types
| Weapon | Behavior | Notes |
|--------|----------|-------|
| **Single** (current) | One shot up | Baseline |
| **Spread / Shotgun** | 3–5 shots fanned at ±10–20° | Short range feel; strong up close |
| **Rapid** | Same shot, shorter cooldown | Just tune the `+ 8` gate |
| **Twin / Wide** | Two parallel shots offset on X | Covers more lanes |
| **Piercing beam** | Fast, doesn't despawn on first hit | Needs "already hit" tracking so it clears a column |
| **Homing missile** | Slower, steers toward nearest enemy each tick | A dose of AI on the *player* side; needs a target-seek pass |
| **Charge shot** | Hold to charge, release for a big slow shot | Needs a charge timer + a variable-radius laser |

### Upgrades / power-ups
Introduce a pickup entity (`EntityType::Powerup`) that drifts down like an enemy;
colliding with the player applies an effect. Reuses spawn + collision plumbing.

- **Weapon swap** — grants Spread / Rapid / Twin for a duration or until hit.
- **Fire-rate up** — permanent-ish cooldown reduction (stackable, capped).
- **Shield refill / +1** — you already track `shield` (max 3); a pickup tops it up
  or raises the cap.
- **Multiplier / bonus** — temporary score multiplier (ties into `score`/`altitude`).
- **Bomb / screen-clear** — consumable that destroys or damages all on-screen enemies.
- **Extra life** — respawn stock, if you add lives later.

### Design notes
- **Timed vs. tiered:** timed power-ups (revert after N frames) keep tension and are
  easy to reason about; tiered/persistent upgrades (lose a level when hit, à la
  Gradius) reward survival but need more state and UI.
- **Drop source:** power-ups can drop from killed enemies (esp. **Tank**/**Splitter**)
  or spawn on a timer like asteroids. Enemy drops make combat feel rewarding.
- **UI:** current weapon + active buffs need a spot in `game_ui.rs` HUD.
- **Smallest first slice:** a `Weapon` enum + a **Spread** shot, toggled by a
  pickup entity — proves out both the weapon-swap and the power-up plumbing at once.

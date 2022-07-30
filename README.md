# Connect 4 CLI

play a casual game of connect 4 in your terminal. Writen in Rust.

# Tasks
- [x] Game Logic Implementation
- [x] Graphics Engine with ASCII game state and menu logic
- [ ] Networking Logic (with `libp2p`, see [tutorial](https://blog.logrocket.com/libp2p-tutorial-build-a-peer-to-peer-app-in-rust) for more info)

# Roadmap
- [ ] P2P User experience
  - [ ] input your name
  - [ ] START or JOIN a game
    - [ ] start => joins room of open games and waits for someone to join...
    - [ ] join  => display all open games! (can select a peer or refresh the page)
  - [ ] make your move... (check for invalid moves)
  - [ ] valid move 
        1. update lokal game state
        2. send move to other client as payload
  - [ ] wait for move from friend

# OVERKILL FEATURES 
- [ ] Brutal Mode => both player have same color, remember your position you must
- [ ] animate drop

# Graphics Engine Example
## Menu Engine (joining a game)
**Screen 0**
```txt
=== connect-4-cli ===

Welcome! (intro text goes here)

Whats your name?
>
```

**Screen 1**
```txt
=== connect-4-cli ===
Welceome dcts

> (1) Start a Game
> (2) Join a Game

Choose action by typing a number:
>
```

**Screen 2**
```txt
=== connect-4-cli ===

✅ game created
joining lobby...

LOBBY
    NAME               | ID
(*) dcts...............| hceui17e8
    art_brock..........| lkv32v873
    frodo..............| sv7823uzd
    gandalf............| v824hjfvx
        
👀 waitig for someone to join your game ... 👀
```

## Playing a Game
- Idea how to structure it
  1. title & info
  2. game state
  3. menu (or loading screen)

```txt
=== connect-4-cli ===

🟡 dcts 
🔴 gandalf 

|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  |  |🔴|  |🟡|🟡|  |
|  |🟡|🟡|🔴|🔴|🔴|🟡|
*--*--*--*--*--*--*--*

waiting for gandalf to play...
>
```

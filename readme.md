Feature Roadmap:
- NL Holdem and 5 Card Draw playable cli game
- Passable AI for game
- Monte carlo simulations for optimal 5CD draws
- Hand/situational statistical analysis tool
- GUI in electron or something, to try out using Rust FFI (or compiling to WASM)
- Other games to support - Limit Holdem, PLO, Omaha Hi Lo, Seven Card Stud

Technical todos for v0.1 Release:
- [x] Tiebreak logic
- [x] Find the best 5 card hand in 7 card holding
- [x] Player and betting logic
- [x] Holdem Game loop
- [ ] Game loop tests
- [ ] Track players with pointers not by idx - Box? RC? Table abstraction? circular list? 
- [ ] Un-objectify GameLoop
- [ ] Game client layer
- [ ] Game CLI interface
- [x] Dummy AI
- [ ] Genericize game loop for multiple game types


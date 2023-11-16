『ゲームで学ぶ探索アルゴリズム実践入門』 https://gihyo.jp/book/2023/978-4-297-13360-3

## 文脈のある一人ゲーム

```
$ cargo run --bin one_player_maze --release
```

- [x] ランダム
- [x] 貪欲法
- [x] ビームサーチ
- [x] Chokudaiサーチ

## 文脈のない一人ゲーム

```
$ cargo run --bin auto_move_maze --release
```

- [x] ランダム
- [x] 山登り
- [x] 焼きなまし

## 交互着手二人ゲーム

```
$ cargo run --bin alternate_maze --release
```

- [x] ランダム
- [x] MiniMax
- [x] AlphaBeta
- [x] 反復深化
- [x] 原始モンテカルロ法
- [x] MCTS (モンテカルロ木探索)
- [x] Thunderサーチ

## 同時着手二人ゲーム

- [ ] ランダム
- [ ] 原始モンテカルロ法
- [ ] MCTS (モンテカルロ木探索)
- [ ] DUCT

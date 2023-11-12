pub trait HeuristicGameState: Clone {
    // 初期解を生成する
    fn initialize(&mut self);
    // ゲームを最後まで進めてスコアを返す
    fn start(&self) -> u32;
}

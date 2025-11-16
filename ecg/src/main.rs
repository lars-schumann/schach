fn main() {
    let game = schach::game::GameState::new();
    for mv in game.core.legal_moves() {
        println!("{mv:?}");
    }
}

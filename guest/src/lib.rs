use interface::Game;

#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    unsafe {
        print_wasm(left);
        print_wasm(right);

        let str = format!("adding {} to {}", left, right);
        accept_str(str.as_ptr(), str.len() as u32);
    }
    left + right
}

#[no_mangle]
pub fn play_game(game: &Game) -> u8 {
    game.data[2] * game.data[31]
}

extern "C" {
    fn print_wasm(a: i32);
    fn accept_str(ptr: *const u8, len: u32);
}

use std::{fs::File, io::Read, path::PathBuf, ptr::addr_of};

use interface::*;
use wasmer::{
    imports, Bytes, Function, FunctionEnv, FunctionEnvMut, Instance, Memory, Module, Pages, Store,
    Value, WasmPtr,
};

struct MyEnv {
    memory: Option<Memory>,
}

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).expect("expected path");
    let path = PathBuf::from(path);
    let mut file = File::open(path)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    let mut store = Store::default();
    let module = Module::new(&store, bytes)?;
    let env = FunctionEnv::new(&mut store, MyEnv { memory: None });

    let print_wasm_typed = Function::new_typed(&mut store, print_wasm);
    let accept_str_typed = Function::new_typed_with_env(&mut store, &env, accept_str);

    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {
        "env" => {
            "print_wasm" => print_wasm_typed,
            "accept_str" => accept_str_typed,
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let env = env.as_mut(&mut store);
    let mem = instance.exports.get_memory("memory")?;
    env.memory = Some(mem.clone());

    let add_one = instance.exports.get_function("add")?;
    let play_game = instance
        .exports
        .get_typed_function::<u32, u8>(&store, "play_game")?;

    let pages: Pages = Bytes(std::mem::size_of::<Game>()).try_into().unwrap();
    let offset_game = mem.grow(&mut store, pages)?.0;

    let game = Game { data: [2; 32] };
    let game_ref = addr_of!(game) as *const u8;
    let game_ref = unsafe { std::slice::from_raw_parts(game_ref, std::mem::size_of::<Game>()) };

    mem.view(&store).write(offset_game.into(), game_ref)?;

    let result = play_game.call(&mut store, offset_game)?;
    dbg!(&result);

    Ok(())
}

fn print_wasm(a: i32) {
    println!("{a}");
}

fn accept_str(env: FunctionEnvMut<MyEnv>, ptr: WasmPtr<u8>, len: u32) {
    let memory = env.data().memory.as_ref().unwrap();
    let memory = memory.view(&env);
    let data = ptr.read_utf8_string(&memory, len).unwrap();
    println!("{data}");
}

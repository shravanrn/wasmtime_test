use wasmtime::*;
use wasmtime_wasi::Wasi;
use wasi_cap_std_sync::WasiCtxBuilder;

use std::env;
use std::io;
use std::path::PathBuf;

fn wasm_path() -> io::Result<PathBuf> {
    let mut p = env::current_exe()?;
    p.pop();
    p.pop();
    p.pop();
    p.push("test.wasm");
    Ok(p)
}

fn main() {
    let wasm_path = wasm_path().unwrap();
    let wasm_str_path = wasm_path.to_str().unwrap();

    // create wasm instance
    let store = Store::default();
    let mut linker = Linker::new(&store);
    let wasi = Wasi::new(
        &store,
        WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args().unwrap()
            .build().unwrap(),
    );
    wasi.add_to_linker(&mut linker).unwrap();

    let module = Module::from_file(store.engine(), wasm_str_path).unwrap();
    linker.module("", &module).unwrap();
    let instance = linker.instantiate(&module).unwrap();

    // call function that mallocs a pointer in the wasm instance and writes 42 to it
    let f = instance.get_func("malloc_and_write").unwrap();
    let ret = f.call(&vec![]).unwrap();
    let index = (*ret).first().map(|a| a.clone()).unwrap().unwrap_i32();

    // compute address of index from host
    let m = instance.get_memory("memory").unwrap();
    let heap_base = m.data_ptr();
    let obj_ptr = unsafe { heap_base.add(index as usize) } as *mut u32;

    println!("Heap base found: {:?}. Object pointer: {:?}", heap_base, obj_ptr);

    let val = unsafe { *obj_ptr };
    println!("Val expected: 42. Val read: {}", val);
}

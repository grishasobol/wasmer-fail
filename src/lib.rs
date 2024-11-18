// These tests must stuck in a loop on M1 apple processor and singlepass compiler.

// #[test]
// fn test_popcnt_i32() {
//     use wasmer::{imports, Instance, Module, Store, Value};

//     let get_next_number_i32 = |mut num: i32| {
//         num ^= num << 13;
//         num ^= num >> 17;
//         num ^= num << 5;
//         num
//     };

//     let module_wat = r#"
//     (module
//         (func $popcnt (export "popcnt") (param i32) (result i32)
//             local.get 0
//             i32.popcnt
//         )
//     )"#;

//     let store = Store::default();
//     let import_object = imports! {};
//     let module = Module::new(&store, &module_wat).unwrap();
//     let instance = Instance::new(&module, &import_object).unwrap();

//     let popcnt = instance.exports.get_function("popcnt").unwrap();

//     let mut num = 1;
//     for _ in 1..10000 {
//         let result = popcnt.call(&[Value::I32(num)]).unwrap();
//         assert_eq!(&Value::I32(num.count_ones() as i32), result.get(0).unwrap());
//         num = get_next_number_i32(num);
//     }
// }

// #[test]
// fn test_popcnt_i64() {
//     use wasmer::{imports, Instance, Module, Store, Value};

//     let get_next_number_i64 = |mut num: i64| {
//         num ^= num << 34;
//         num ^= num >> 40;
//         num ^= num << 7;
//         num
//     };

//     let module_wat = r#"
//     (module
//         (func $popcnt (export "popcnt") (param i64) (result i64)
//             local.get 0
//             i64.popcnt
//         )
//     )"#;

//     let store = Store::default();
//     let import_object = imports! {};
//     let module = Module::new(&store, &module_wat).unwrap();
//     let instance = Instance::new(&module, &import_object).unwrap();

//     let popcnt = instance.exports.get_function("popcnt").unwrap();

//     let mut num = 1;
//     for _ in 1..10000 {
//         let result = popcnt.call(&[Value::I64(num)]).unwrap();
//         assert_eq!(&Value::I64(num.count_ones() as i64), result.get(0).unwrap());
//         num = get_next_number_i64(num);
//     }
// }

// #[test]
// fn test_ctz() {
//     use wasmer::{imports, Instance, Module, Store, Value};
//     let module_wat = r#"
//     (module
//         (func (export "lol") (param i32) (result i32)
//             i32.const 0xDEADBEEF
//             i32.ctz
//             (block
//                 (block
//                     (block
//                         br 1
//                         i32.const 0xdeadbaaf
//                         i32.ctz
//                         drop
//                     )
//                     i32.const 0xdeadbaac
//                     i32.ctz
//                     drop
//                 )
//                 i32.const 0xdeadfaaf
//                 i32.ctz
//                 drop
//             )
//             drop
//             i32.const 0xDEADBEEF
//             i32.ctz
//         )
//     )"#;

//     let store = Store::default();
//     let import_object = imports! {};
//     let module = Module::new(&store, &module_wat).unwrap();
//     let instance = Instance::new(&module, &import_object).unwrap();

//     let lol = instance.exports.get_function("lol").unwrap();
//     println!("{:?}", lol.call(&[Value::I32(10)]));
// }

// #[test]
// fn check_wasm_execution_stack() {
//     use wasmer::{imports, Instance, Module, Store, Value};
//     let module_wat = r#"
//     (module
//         (func (export "lol") (param i64) (result i64)
//             global.get 0
//             local.get 0
//             i64.add
//             global.set 0
//             (call 0 (local.get 0))
//             drop
//             global.get 0
//         )
//         (global (mut i64) (i64.const 1))
//         (export "global" (global 0))
//     )
//     "#;

//     let store = Store::default();
//     let import_object = imports! {};
//     let module = Module::new(&store, &module_wat).unwrap();
//     let instance = Instance::new(&module, &import_object).unwrap();

//     let lol = instance.exports.get_function("lol").unwrap();
//     println!("{:?}", lol.call(&[Value::I64(1)]));
//     println!("{:?}", instance.exports.get_global("global"));
// }

// #[test]
// fn check_wasm_execution_stack1() {
//     use wasmer::{imports, Function, Instance, Module, Store, Value};
//     let module_wat = r#"
//     (module
//         (import "env" "add" (func $add (param i64 i64) (result i64)))
//         (func (export "lol") (param i64) (result i64)
//             (call 0 (global.get 0) (local.get 0))
//             (call 0 (global.get 0))
//             (call 0 (global.get 0))
//             (call 0 (global.get 0))
//             global.set 0
//             (call 1 (local.get 0))
//         )
//         (global (mut i64) (i64.const 1))
//         (export "global" (global 0))
//     )
//     "#;

//     let store = Store::default();
//     let add = |x: i64, y: i64| x + y;
//     let import_object = imports! {
//         "env" => {
//             "add" => Function::new_native(&store, add)
//         },
//     };
//     let module = Module::new(&store, &module_wat).unwrap();
//     let instance = Instance::new(&module, &import_object).unwrap();

//     let lol = instance.exports.get_function("lol").unwrap();
//     println!("{:?}", lol.call(&[Value::I64(1)]));
//     println!("{:?}", instance.exports.get_global("global"));
// }

// #[test]
// fn check_wasm_execution_stack2() {
//     use wasmer::{imports, Function, Instance, Module, Store};

//     let wat = std::fs::read("code.wat").unwrap();

//     let mut store = Store::default();
//     let import_object = imports! {
//         "env" => {
//             "import" => Function::new_typed(&mut store, |_x: i32, _y: i32, _z: i32| {}),
//         },
//     };
//     let module = Module::new(&store, &wat).unwrap();
//     let instance = Instance::new(&mut store, &module, &import_object).unwrap();

//     let init = instance.exports.get_function("init").unwrap();
//     println!("{:?}", init.call(&mut store, &[]));
//     println!("{:?}", instance.exports.get_global("gas").unwrap().get(&mut store));
// }

#[test]
fn check_unreachable() {
    use wasmer::{imports, Instance, Module, Store};
    let module_wat = r#"
    (module
        (func (export "func")
            i32.const 10
            i32.const 20
            i32.add
            drop
        )
    )"#;

    let mut store = Store::default();
    let import_object = imports! {};
    let module = Module::new(&store, &module_wat).unwrap();
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let lol = instance.exports.get_function("func").unwrap();
    println!("{:?}", lol.call(&mut store, &[]));
}

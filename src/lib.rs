// These tests must stuck in a loop on M1 apple processor and singlepass compiler.

#[test]
fn test_i32() {
    use wasmer::{Store, Module, Instance, imports, Value};

    let get_next_number_i32 = |mut num: i32| {
        num ^= num << 13;
        num ^= num >> 17;
        num ^= num << 5;
        num
    };

    let module_wat = r#"
    (module
        (func $popcnt (export "popcnt") (param i32) (result i32)
            local.get 0
            i32.popcnt
        )
    )"#;

    let store = Store::default();
    let import_object = imports! {};
    let module = Module::new(&store, &module_wat).unwrap();
    let instance = Instance::new(&module, &import_object).unwrap();

    let popcnt = instance.exports.get_function("popcnt").unwrap();

    let mut num = 1;
    for _ in 1..10000 {
        let result = popcnt.call(&[Value::I32(num)]).unwrap();
        assert_eq!(&Value::I32(num.count_ones() as i32), result.get(0).unwrap());
        num = get_next_number_i32(num);
    }
}

#[test]
fn test_i64() {
    use wasmer::{Store, Module, Instance, imports, Value};

    let get_next_number_i64 = |mut num: i64| {
        num ^= num << 34;
        num ^= num >> 40;
        num ^= num << 7;
        num
    };

    let module_wat = r#"
    (module
        (func $popcnt (export "popcnt") (param i64) (result i64)
            local.get 0
            i64.popcnt
        )
    )"#;

    let store = Store::default();
    let import_object = imports! {};
    let module = Module::new(&store, &module_wat).unwrap();
    let instance = Instance::new(&module, &import_object).unwrap();

    let popcnt = instance.exports.get_function("popcnt").unwrap();

    let mut num = 1;
    for _ in 1..10000 {
        let result = popcnt.call(&[Value::I64(num)]).unwrap();
        assert_eq!(&Value::I64(num.count_ones() as i64), result.get(0).unwrap());
        num = get_next_number_i64(num);
    }
}

#[test]
fn test1() {
    use wasmer::{Store, Module, Instance, imports, Value};
    let module_wat = r#"
    (module
        (func (export "lol") (param i32) (result i32)
            i32.const 0xDEADBEEF
            i32.ctz
            (block
                (block
                    (block
                        br 1
                        i32.const 0xdeadbaaf
            i32.ctz
                        drop
                    )
                    i32.const 0xdeadbaac
            i32.ctz
                    drop
                )
                i32.const 0xdeadfaaf
            i32.ctz
                drop
            )
            drop
            i32.const 0xDEADBEEF
            i32.ctz
        )
    )"#;

    let store = Store::default();
    let import_object = imports! {};
    let module = Module::new(&store, &module_wat).unwrap();
    let instance = Instance::new(&module, &import_object).unwrap();

    let lol = instance.exports.get_function("lol").unwrap();
    println!("{:?}", lol.call(&[Value::I32(10)]));
}
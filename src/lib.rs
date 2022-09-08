#[cfg(test)]
mod mod_test {
    extern crate libc;
    extern crate nix;
    extern crate wasmer;

    use self::libc::{c_void, siginfo_t};
    use self::nix::sys::signal::{self, SigHandler};
    use self::wasmer::{imports, Instance, Module};

    static mut MEM: *mut c_void = core::ptr::null_mut();
    static mut MEM_SIZE: usize = 0;
    static mut OLD_SIGHANDLER: Option<SigHandler> = None;

    extern "C" fn handle_sigsegv(x: i32, y: *mut siginfo_t, z: *mut c_void) {
        unsafe {
            println!("!!! receive signal : {}", x);
            let mem = (*y).si_addr();
            println!("mem = {:?}", mem);
            let ps = page_size::get();
            if mem >= MEM && (mem as usize - MEM as usize) < MEM_SIZE {
                if libc::mprotect(MEM, ps, libc::PROT_WRITE) != 0 {
                    panic!("mprotect cannot allows write access");
                }
            } else if let Some(old_handler) = OLD_SIGHANDLER {
                match old_handler {
                    SigHandler::SigDfl | SigHandler::SigIgn => panic!("unsupported"),
                    SigHandler::Handler(func) => func(x),
                    SigHandler::SigAction(func) => func(x, y, z),
                }
            }
        }
    }

    #[test]
    fn test() {
        let compiler = wasmer::Singlepass::default();
        let store = wasmer::Store::new(&wasmer::Universal::new(compiler).engine());

        unsafe {
            let sig = if cfg!(target_os = "macos") {
                signal::SIGBUS
            } else {
                signal::SIGSEGV
            };
            let handler = signal::SigHandler::SigAction(handle_sigsegv);
            let sig_action = signal::SigAction::new(
                handler,
                signal::SaFlags::SA_SIGINFO,
                signal::SigSet::empty(),
            );
            OLD_SIGHANDLER.replace(
                signal::sigaction(sig, &sig_action)
                    .expect("Cannot set sigaction")
                    .handler(),
            );
        }

        let wat = r#"(module (func call 0) (export "init" (func 0)))"#;

        let module = Module::new(&store, wat).unwrap();
        let import_object = imports! {};
        let instance = Instance::new(&module, &import_object).unwrap();

        let init = instance.exports.get_function("init").unwrap();
        let result = init.call(&[]);
        println!("{:?}", result);
    }
}

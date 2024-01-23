#![deny(
    unsafe_code,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]


use std::ffi::CStr;

use sm_ext::{forwards, native, ExecType, ICallableApi, IExtension, IExtensionInterface, IForwardManager, IShareSys, SMExtension, SMInterfaceApi};

#[derive(Default, SMExtension)]
#[extension(name = "shrimpstats_tf2", description = "ShrimpStats TF2 Extension")]
pub struct ShrimpStatsTF2Extension;

#[forwards]
struct MyForwards {
    /// ```sourcepawn
    /// forward int OnRustCall(int a, int b, const char[] c);
    /// ```
    #[global_forward("OnRustCall", ExecType::Hook)]
    on_rust_call: fn(a: i32, b: i32, c: &CStr) -> i32,
}


impl IExtensionInterface for ShrimpStatsTF2Extension {
    fn on_extension_load(&mut self, me: IExtension, sys: IShareSys, late: bool) -> Result<(), Box<dyn std::error::Error>> {
        println!(">>> Rusty extension loaded! me = {:?}, sys = {:?}, late = {:?}", me, sys, late);

        let forward_manager: IForwardManager = sys.request_interface(&me)?;
        println!(">>> Got interface: {:?} v{:?}", forward_manager.get_interface_name(), forward_manager.get_interface_version());

        Ok(())
    }

    fn on_extension_unload(&mut self) {}

    fn on_extensions_all_loaded(&mut self) {
    }
}
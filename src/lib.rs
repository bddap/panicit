#![no_std]
#![feature(core_intrinsics)]

#[no_mangle]
fn call_abort() {
    unsafe { core::intrinsics::abort() }
}

#[no_mangle]
fn call_unreachable() {
    unsafe { core::arch::wasm32::unreachable() }
}

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn on_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

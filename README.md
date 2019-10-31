This demonstration shows that in when building for wasm32-unknown-unknown,
`core::intrinsics::abort()` is equivalent to `core::arch::wasm32::unreachable()`.

## Instructions

Make sure you have installed:
- rust nightly
- wasm2wat

Run `./show`.

## Spoiler

This is what get printed:

```wat
(module
  (type (;0;) (func))
  (func $call_abort (type 0)
    unreachable
    unreachable)
  (func $call_unreachable (type 0)
    unreachable
    unreachable)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (export "call_abort" (func $call_abort))
  (export "call_unreachable" (func $call_unreachable)))
```

You'll notice that `func $call_abort` and `func $call_unreachable` are equivalent.

## Note

While behavior is effectively the same, when compiling without optimizations the result is slightly
different.

```wat
(module
  (type (;0;) (func))
  (func $_ZN4core9core_arch6wasm3211unreachable17hb97c2dda9be9f0d9E (type 0)
    unreachable
    unreachable)
  (func $call_abort (type 0)
    unreachable
    unreachable)
  (func $call_unreachable (type 0)
    call $_ZN4core9core_arch6wasm3211unreachable17hb97c2dda9be9f0d9E
    unreachable)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (export "call_abort" (func $call_abort))
  (export "call_unreachable" (func $call_unreachable)))
```

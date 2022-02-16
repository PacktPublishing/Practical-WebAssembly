(module

     (global $mutableValue (import "js" "mutableGlobal") (mut i32))

     (global $immutableValue (import "js" "immutableGlobal") i32)

     (global $wasmValue i32 (i32.const 10))

     (func (export "getWasmValue") (result i32)
        (global.get $wasmValue))

     (func (export "getMutableValue") (result i32)
        (global.get $mutableValue))

	 (func (export "getImmutableValue") (result i32)
        (global.get $immutableValue))

     (func (export "setMutableValue") (param $v i32)
        (global.set $mutableValue
            (local.get $v)))
)

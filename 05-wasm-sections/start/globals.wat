(module
     (global $mutableValue (import "js" "mutableGlobal") (mut i32))

     (global $immutableValue (import "js" "immutableGlobal") i32)

     (func (export "getMutableValue") (result i32)
          (global.get $mutableValue))

	 (func (export "getImmutableValue") (result i32)
        (global.get $immutableValue))

     (func $setMutableValue (export "setMutableValue") (param $v i32)
          (global.set $mutableValue
               (local.get $v)))

     (func $initMutableValue
          (global.set $mutableValue
               (i32.const 200)))

     (start $initMutableValue)
)
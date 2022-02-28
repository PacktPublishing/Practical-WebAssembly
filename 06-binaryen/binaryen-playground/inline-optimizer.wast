(module
    (func $parent (export "parent") (result i32)
        (i32.add
            (call $child)
            (i32.const 13)
        )
    )
    (func $child (result i32) (call $grandChild))
    (func $grandChild (result i32) (call $greatGrandChild))
    (func $greatGrandChild (result i32) (i32.const 7))
)

(module $single-i32_gt_u

    (func $i32_gt_u (param i32 i32) (result i32)
        (local.get 0) (local.get 1) (i32.gt_u)
    )

    (export "i32_gt_u" (func $i32_gt_u))
)

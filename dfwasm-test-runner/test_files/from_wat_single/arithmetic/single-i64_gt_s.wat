(module $single-i64_gt_s

    (func $i64_gt_s (param i64 i64) (result i32)
        (local.get 0) (local.get 1) (i64.gt_s)
    )

    (export "i64_gt_s" (func $i64_gt_s))
)

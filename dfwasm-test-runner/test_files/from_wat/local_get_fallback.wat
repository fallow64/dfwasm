(module
  (func $local_get_fallback (export "local_get_fallback") (result i32)
    (local $l1 i32)
    local.get $l1 ;; get a local varaible that has not been set
  )
)
(module
  (memory $memory (export "memory") 42)

  (func $memory_size (export "memory_size") (result i32)
    memory.size
  )
)

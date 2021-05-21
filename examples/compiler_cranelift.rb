require_relative "prelude"

# A Wasm module can be compiled with multiple compilers.
#
# For the moment, only the Cranelift is available, and it's set and
# used by default. You have nothing to do.
#
# You can run the example directly by executing in Wasmer root:
#
# ```shell
# $ ruby examples/compiler_cranelift.rb
# ```
#
# Ready?

# Let's declare the Wasm module with the text representation.
wasm_bytes = Wasmer::wat2wasm(
  (<<~WAST)
  (module
    (type $sum_t (func (param i32 i32) (result i32)))
    (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
      local.get $x
      local.get $y
      i32.add)
    (export "sum" (func $sum_f)))
  WAST
)

# Create a store, that holds the engine, that holds the compiler.
store = Wasmer::Store.new

# Here we go.
#
# Let's compile the Wasm module. It is at this step that the Wasm text
# is transformed into Wasm bytes (if necessary), and then compiled to
# executable code by the compiler, which is then stored in memory by
# the engine.
module_ = Wasmer::Module.new store, wasm_bytes

# Congrats, the Wasm module is compiled! Now let's execute it for the
# sake of having a complete example.
#
# Let's instantiate the Wasm module.
instance = Wasmer::Instance.new module_, nil

# The Wasm module exports a function called `sum`.
sum = instance.exports.sum
results = sum.(1, 2)

assert { results == 3 }

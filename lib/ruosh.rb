require 'ffi'

module Ruosh
  extend FFI::Library
  ffi_lib './target/debug/libruosh.dylib'

  attach_function :init_my_things, [], :void
end

Ruosh.init_my_things

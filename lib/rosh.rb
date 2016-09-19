require 'ffi'

module Rosh
  extend FFI::Library
  ffi_lib './target/debug/librosh.dylib'

  attach_function :init_my_things, [], :void
end

Rosh.init_my_things

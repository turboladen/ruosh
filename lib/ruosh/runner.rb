module Ruosh
  class Runner
    attr_reader :binding

    def initialize
      @binding = Kernel.binding
    end

    def run(code)
      puts "Running code:\n#{code}"
      Kernel.eval(code, @binding)
    rescue => ex
      puts "ex: #{ex.class}, #{ex.message}"
      ex
    end
  end
end

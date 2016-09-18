module Ruosh
  class Runner
    attr_reader :last_error, :command_history, :result_history, :input_history

    def initialize
      @binding = Kernel.binding
      @last_error = nil
      @command_number = 0
      @command_history = []
      @result_history = []
      @input_history = []

      # Set binding-specific $last_result
      eval('$last_result = nil', @binding)
    end

    # @param code [String]
    # @return [Object]
    def run(code)
      @input_history << code
      code_chunks = split_from_pipes(code)
      final_result = nil

      code_chunks.each_with_index do |code_chunk, i|
        @command_number += 1

        code_chunk = %[$last_result.instance_eval("#{code_chunk}")] unless i.zero?

        final_result = run_code_chunk(code_chunk, @command_number)
      end

      final_result
    end

    def last_result
      @result_history[-2]
    end

    def last_command
      @command_history[-2]
    end

    private

    # @param code [String]
    # @return [Array]
    def split_from_pipes(code)
      code.split('|>')
    end

    # @param code [String]
    # @param cmd_number [Fixnum]
    # @return [Ruosh::Result]
    def run_code_chunk(code, cmd_number)
      command = Command.new(code, cmd_number)
      @command_history.push(command)
      result_object = command.run(@binding)

      result = Ruosh::Result.new(result_object, cmd_number)
      @result_history.push(result)
      @last_error = result if result.error?

      result
    end
  end

  class Command
    attr_reader :code, :command_number

    def initialize(code, command_number)
      @code = code
      @command_number = command_number
    end

    def run(binding)
      Kernel.eval("$last_result = #{@code}", binding)
    rescue => ex
      ex
    end
  end
end

package _2019.intcode

enum class OpCode(val value: Int) {
    ADD(1),
    MULTIPLY(2),
    HALT(99);

    companion object {
        fun getForOpCode(opcode: Int) = values().first { it.value == opcode }
    }
}

enum class Status {
    READY,
    HALTED
}

class IntcodeComputer(program: List<Int>) {
    private val memory = mutableMapOf<Int, Int>()
    private val stdin = mutableListOf<Int>()
    private val stdout = mutableListOf<String>()
    private var instructionPointer = 0
    private var status = Status.READY

    init {
        program.forEachIndexed { index, value -> memory[index] = value }
    }

    fun runUntilBlocked() {
        while (status == Status.READY) {
            when (OpCode.getForOpCode(readInstruction())) {
                OpCode.ADD -> handleAddOpCode()
                OpCode.MULTIPLY -> handleMultiplyOpCode()
                OpCode.HALT -> handleHaltOpCode()
            }
        }
    }

    fun getMemory() = memory.toMap()
    fun getMemoryDangerously() = memory

    private fun readInstruction(): Int {
        val result = memory.getOrElse(instructionPointer, { OpCode.HALT.value })
        instructionPointer += 1
        return result
    }

    private fun handleAddOpCode() {
        val result = listOf(readInstruction(), readInstruction())
            .map { memory.getValue(it) }
            .sum()

        memory[readInstruction()] = result
    }

    private fun handleMultiplyOpCode() {
        val result = listOf(readInstruction(), readInstruction())
                .map { memory.getValue(it) }
                .reduce { a, b -> a * b }

        memory[readInstruction()] = result
    }

    private fun handleHaltOpCode() {
        status = Status.HALTED
    }
}

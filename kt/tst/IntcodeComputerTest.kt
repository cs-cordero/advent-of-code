import _2019.intcode.IntcodeComputer
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.TestFactory

internal class IntcodeComputerTest {

    @TestFactory
    fun `Basic Intcode Tests from Day 2`() = listOf(
        Pair(listOf(1, 0, 0, 0, 99), listOf(2, 0, 0, 0, 99)),
        Pair(listOf(1, 0, 0, 0, 99), listOf(2, 0, 0, 0, 99)),
        Pair(listOf(2, 4, 4, 5, 99, 0), listOf(2, 4, 4, 5, 99, 9801)),
        Pair(listOf(1, 1, 1, 4, 99, 5, 6, 0, 99), listOf(30, 1, 1, 4, 2, 5, 6, 0, 99))
    ).map {
        DynamicTest.dynamicTest("${it.first} should become ${it.second}") {
            val computer = IntcodeComputer(it.first)
            computer.runUntilBlocked()

            val memory = computer.getMemory()
            it.second.forEachIndexed { index, value -> assertEquals(memory[index], value) }
        }
    }
}

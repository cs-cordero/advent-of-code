package _2019.day02

import _2019.intcode.IntcodeComputer
import java.io.File

fun main() {
    val data = File("./kt/2019/day02/input.txt")
            .readText()
            .split(",")
            .map { it.toInt() }

    println("Solution 1: ${solution1(data)}")
    println("Solution 2: ${solution2(data)}")
}

fun solution1(program: List<Int>, noun: Int = 12, verb: Int = 2): String {
    val computer = IntcodeComputer(program)
    val memory = computer.getMemoryDangerously()
    memory[1] = noun
    memory[2] = verb
    computer.runUntilBlocked()
    return computer.getMemory().getValue(0).toString(10)
}

fun solution2(program: List<Int>): String {
    (1..100).forEach { noun ->
        (1..100).forEach { verb ->
            if (solution1(program, noun, verb) == "19690720") {
                return ((noun * 100) + verb).toString()
            }
        }
    }
    return "Unable to find noun-verb combo!"
}

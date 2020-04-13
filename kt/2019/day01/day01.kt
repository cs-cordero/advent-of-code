package _2019.day01

import java.io.File

fun main() {
    val lines = File("./kt/2019/day01/input.txt").readLines()
    println("Solution 1: ${solution1(lines)}")
    println("Solution 2: ${solution2(lines)}")
}

fun solution1(lines: List<String>): String {
    return lines.map { it.toInt() / 3 - 2 }
                .sum()
                .toString()
}

fun solution2(lines: List<String>): String {
    return lines.map(::recursiveFuel).sum().toString()
}

private fun recursiveFuel(fuel: String): Int {
    var result = 0
    var currentFuel = fuel.toInt()
    while (currentFuel > 0) {
        currentFuel = (currentFuel / 3 - 2).coerceAtLeast(0)
        result += currentFuel
    }
    return result
}

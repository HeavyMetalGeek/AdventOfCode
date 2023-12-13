import kotlin.math.pow

fun main() {
    getResourceAsText("/input.txt")?.lines()?.sumOf { line ->
        val (card, winList, haveList) = line.split(":", "|")
        val wins = winList.split(" ").filter { it != "" }.map { it.trim().toInt() }
        val haves = haveList.split(" ").filter { it != "" }.map { it.trim().toInt() }
        // Bit shift to get the 0, 1, 2, 4, 8 sequence for inputs of 0, 1, 2, 3, 4
        haves.count { wins.contains(it) }.let { 1 shl (it - 1) }
    }?.also {
        println("You've won $it somethings!")
    }
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()

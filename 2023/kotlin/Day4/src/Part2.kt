import kotlin.math.pow

fun main() {
    getResourceAsText("/input.txt")?.lines()?.map { line ->
        val (card, winList, haveList) = line.split(":", "|")
        val wins = winList.split(" ").filter { it != "" }.map { it.trim().toInt() }
        val haves = haveList.split(" ").filter { it != "" }.map { it.trim().toInt() }
        haves.count { wins.contains(it) }
    }?.also { winCounts ->
        val cards = IntArray(winCounts.size) { 1 }
        // Accumulate card copies by snowballing
        winCounts.withIndex().sumOf { (i, wins) ->
            (1..wins).forEach { cards[i + it] += cards[i] }
            cards[i]
        }.also {
            println("You'd have $it scratchcards")
        }
    }
}

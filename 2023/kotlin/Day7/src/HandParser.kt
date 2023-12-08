/** Convert input text into list of Hand objects */
fun parseHand(strengths: String, input: String): List<Hand> {
    val regex = Regex("(\\w+) (\\d+)")
    return input.lines().map {
        val match = regex.find(it)!!
        val (cards, bid) = match.destructured
        // Optimize card chars to strengths (e.g. A -> 0, K -> 1, ...)
        val mapped = cards.map { c -> strengths.indexOf(c) }
        Hand(cards, mapped, bid.toInt())
    }
}

/** Print result to console */
fun List<Hand>.printWinnings(debug: Boolean = true) {
    var sum = 0
    // Reversed because we sorted strongest->weakest, but rank is weakest->strongest
    reversed().forEachIndexed { rank, hand ->
        sum += (rank + 1) * hand.bid
    }
    if(debug)
        println(this)
    println("Total winnings: $sum")
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
        object {}.javaClass.getResource(path)?.readText()
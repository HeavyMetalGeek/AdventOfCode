/** Goes through each result and returns the max cubes per color per game */
fun parseCubes(input: String) = run {
    val (game, results) = input.split(": ")
    val gameId = game.split(" ")[1].toInt()
    // Break down the list of draws into individual "color: count" maps
    gameId to results.split("; ").map { draws ->
        draws.split(", ").associate { draw ->
            val (count, color) = draw.split(" ")
            color to count.toInt()
        }
    }.let {
        // This merges each map with the previous, keeping only max values when two collide
        mutableMapOf<String, Int>().apply {
            it.forEach {
                it.forEach { (k, v) -> merge(k, v) { oldVal, newVal -> Math.max(oldVal, newVal) } }
            }
        }
    }
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
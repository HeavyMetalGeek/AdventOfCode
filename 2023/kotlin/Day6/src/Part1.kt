fun main() {
    getResourceAsText("input.txt")?.lines()?.map { line ->
        // Separate each row into a list of numbers (by dropping the first text column)
        line.split("\\s+".toRegex()).drop(1).map { it.toInt() }
    }?.let { (times, distances) ->
        // Make a pair for each column like Pair(1st row, 2nd row)
        times zip distances
    }?.map { (time, distance) ->
        // Count the number of items in this range that beat the distance
        (0L..time).count { (time - it) * it > distance }
    }?.reduce { acc, i -> acc * i }?.also {
        println("Product of all ways to win: $it")
    }
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()

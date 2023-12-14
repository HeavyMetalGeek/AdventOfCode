fun main() {
    getResourceAsText("/input.txt")?.split("\n\n", "\r\n\r\n")?.sumOf { section ->
        // Tries to find horizontal reflection; if it can't, tries vertical reflection
        section.lines().map { it.toCharArray() }.run {
            findReflection()?.let { 100 * (it + 1) } // Horizontal; 100 points per row
                ?: rotate().findReflection()?.plus(1) // Vertical; only 1 point per row
                ?: error("Contract of expectation vitiated")
        }
    }?.also {
        println(it)
    }
}

/** Swaps x/y points, effectively converting rows into columns. "Rotate" is actually wrong. */
fun List<CharArray>.rotate() = List(this[0].size) { y ->
    CharArray(size) { x -> this[x][y] }
}

/** A variation of my original reflection finding technique.  Finds the horizontal reflection
 * with a double for-loop.  The loops continue until the *first* match of the errors param. */
fun List<CharArray>.findReflection(requiredErrors: Int = 0) = indices.firstOrNull { i ->
    val max = 2 * (i + 1) - 1 // If i = 1 -> max = 3, so check 2 against 1, and 3 against 0
    (i + 1.. max).sumOf { // Count the number of differences in this back trace
        if(it >= size) 0 else this[it].differCount(this[max - it])
    } == requiredErrors // Part 2: *require* smudges to be considered a reflection
}?.takeIf { it != size - 1 } // If we're at the last index, we found no reflection

/** Simply counts the number of differences between two character arrays. */
infix fun CharArray.differCount(other: CharArray) =
    indices.count { this[it] != other[it] }

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()

// The following was the original implementation, before part 2 when I rewrote it for CharArray.
/** Finds a horizontal reflection.  For each row, work backwards to find the first row that
 * has a mirror.  Any checks that go out of bounds are considered a match for simplicity. */
/* fun List<String>.findReflection() = indices.first { i ->
    val max = 2 * (i + 1) - 1 // If i = 1 -> max = 3, so check 2 against 1, and 3 against 0
    (i + 1.. max).all { it >= size || this[it] == this[max - it] }
}.takeIf { it != size - 1 } // If we're at the last index, we found no reflection */

fun main() {
    getResourceAsText("input.txt")?.lines()?.sumOf { line ->
        val (springs, splitsText) = line.split(" ")
        val splits = splitsText.split(",").map { it.toInt() } // E.g. 3,1,1
        memoizationMap.clear() // Probably should've used a wrapper class
        StepData().bruteSolve(springs.toCharArray(), splits).also {
            // println("$springs $splits = $it") // Debug
        }
    }?.also {
        println("Total sum of possible arrangements: $it")
    }
}

/** So named "bruteSolve" because it is largely a brute force solution saved only by memoization.  The Part 1
 * version of this function was not originally memoized and did not use StepData (using arguments instead). */
fun StepData.bruteSolve(springs: CharArray, splits: List<Int>): Long = memoize {
    if (springIndex == springs.size) // Base case - if we've parsed the whole string
        // This return is complicated; return 1 if we counted the right numbers of damaged springs, 0 otherwise
        return@memoize 1L.takeIf{ splitIndex == splits.size }
            ?: if(splitIndex == splits.size - 1 && broken == splits[splitIndex]) 1L else 0L
    // println("\tD: [$springIndex]=${override ?: springs[springIndex]} split=$splitIndex broke=$broken") // Debug :)
    // The recursive part of this function
    return@memoize when (override ?: springs[springIndex]) {
        // Return 0 if counting this # exceeds the limit for this split, otherwise recurse
        '#' -> 0L.takeIf { splitIndex == splits.size || broken == splits[splitIndex] }
            ?: copy(override = null, springIndex = springIndex + 1, broken = broken + 1)
                .bruteSolve(springs, splits)
        // Return 0 if there aren't enough #'s when we break this split, otherwise recurse
        '.' -> 0L.takeIf { broken > 0 && broken != splits[splitIndex] }
            ?: copy(override = null, springIndex = springIndex + 1, splitIndex = splitIndex + if (broken > 0) 1 else 0, broken = 0)
                .bruteSolve(springs, splits)
        // If we see a '?', try substituting both '#' and '.' in its place
        else -> listOf('#', '.').sumOf { copy(override = it).bruteSolve(springs, splits) }
    }
}

/** Holds the function parameters for recursion and doubles as an object to test for memoization. */
data class StepData(val override: Char? = null, val springIndex: Int = 0, val splitIndex: Int = 0, val broken: Int = 0)
val memoizationMap = mutableMapOf<StepData, Long>()
fun StepData.memoize(call: () -> Long): Long =
    memoizationMap[this] ?: call().also {
        memoizationMap[this] = it
    }

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
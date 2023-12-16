fun main() {
    val requiredCycles = 1000000000 // Holy cats!
    // This cycle map is used to detect the inevitable cycle
    val cycleMap = mutableMapOf<Int, CycleEntry>()
    // Generate a sequence; i.e. lazily loaded list of transformations
    generateSequence(getResourceAsText("/input.txt")?.lines()?.map { it.toCharArray() }) { grid ->
        // Tilt the grid in N, W, S, E directions; we use "apply" here to return the same grid reference
        grid.apply { listOf(Dir.N, Dir.W, Dir.S, Dir.E).forEach { tilt(it) } }
    }.map { grid ->
        // Convert each grid sequence item into its "load" equivalent
        grid.withIndex().sumOf { (y, line) ->
            line.count { it == 'O' } * (grid.size - y)
        }
    }.withIndex().takeUntilInclusive { (step, load) ->
        // A very efficient, but weak way to detect cycles - for each load, remember the last "step" we saw it.
        // As we continue taking, if we see that number again, store the length (this step - last seen step).
        // Continuing, if we see that number a third time, and the length is the same, we've found a cycle.
        val length = step - (cycleMap[load]?.step ?: step) // 0 if this is the first encounter
        (length > 1 && cycleMap[load]?.length == length).also { // '> 1' to prevent 9, 9, 9 from matching.
            // println("cycleMap[$load] = CycleEntry($step, $length)") // Some handy debugging
            cycleMap[load] = CycleEntry(step, length)
        }
    }.toList().also {
        val length = cycleMap[it.last().value]?.length!! // The last item we stopped on knows the cycle length.
        val start = it.last().index - length // Get the first index of the previous cycle (used later)
        val simulatedIndex = start + (requiredCycles - it.last().index) % length // Math is fun :)
        println("Result is ${it[simulatedIndex].value}")
    }
}

data class CycleEntry(val step: Int, val length: Int?)


enum class Dir {
    N, S, E, W
}

/** Given a direction to tilt, move each "O" in that direction until it hits something.  Assumes square grid. */
fun List<CharArray>.tilt(dir: Dir) {
    for(i in indices) { // Virtual "x"
        var swapToI = 0 // The index of the free space swap spot; it doesn't have to be free initially
        for(j in indices) { // Virtual "y"
            val(x, y) = transform(dir, i, j) // Rotate these coordinates based on dir
            when (this[y][x]) {
                'O' -> { // This is a ball and it should roll; we do this by swapping 'O' with '.'
                    val (swapX, swapY) = transform(dir, i, swapToI) // Rotate the swap coordinates
                    val c = this[y][x]; this[y][x] = this[swapY][swapX]; this[swapY][swapX] = c
                    swapToI++ // This spot is now occupied, so move the swap spot over one
                }
                '#' -> swapToI = j + 1 // This is a rock and is solid, so the next swap spot is adjacent
            }
        }
    }
}

/** Return rotated coordinates from the given direction.  Only works on square grids! */
fun List<CharArray>.transform(dir: Dir, x: Int, y: Int) = when(dir) {
    Dir.N -> Pair(x, y)
    Dir.S -> Pair(size - x - 1, size - y - 1)
    Dir.E -> Pair(size - y - 1, x)
    Dir.W -> Pair(y, size - x - 1)
}

/** Takes items in the sequence until the predicate returns true, including the item that returned true. */
fun <T> Sequence<T>.takeUntilInclusive(predicate: (T) -> Boolean): Sequence<T> {
    var delayed = true
    return takeWhile {
        val loop = delayed
        delayed = !predicate(it)
        loop
    }
}
import kotlin.math.abs

/** Expands the galaxy given by the expansion rate (1 = double, 9 = 10x) */
fun galaxyCalculation(inputFile: String, expansionRate: Int) {
    getResourceAsText(inputFile)?.lines()?.let { lines ->
        val horizontalExpansion = BooleanArray(lines[0].length) { true }
        var yExpansionModifier = 0
        // Each bucket holds the temporary "x" value of its respective column of galaxies
        val galaxyBuckets = buildMap<Int, MutableList<Int>> {
            repeat(lines[0].length) { i -> put(i, mutableListOf()) }
        }
        // Put galaxy y-coords into buckets, expanding the galaxy as it goes down
        lines.forEachIndexed { y, line ->
            when(line.all { it == '.' }) {
                true -> yExpansionModifier += expansionRate
                false -> line.forEachIndexed { x, c -> if(c == '#') {
                    horizontalExpansion[x] = false
                    galaxyBuckets[x]?.add(y + yExpansionModifier)
                }}
            }
        }
        // Recalculate the x-coords after expanding
        var xExpansionModifier = 0
        horizontalExpansion.map { b ->
            if(b) xExpansionModifier += expansionRate
            xExpansionModifier
        }.run {
            galaxyBuckets.mapKeys { (k, _) -> k + this[k] }
        }.flatMap { (x, list) ->
            // Convert the separate x/y coords into a Galaxy object, and flatten into one list
            list.map { y -> Galaxy(x, y) }
        }
    }?.apply {
        // Calculate sum of all combinations
        var sum = 0L
        for(i in indices) {
            for(j in i + 1..<size)
                sum += this[i].distanceTo(this[j])
        }
        println("Sum of all distances: $sum")
    }
}

data class Galaxy(val x: Int, val y: Int) {
    // Distance between items in a grid can be represented via Manhattan Distance
    fun distanceTo(other: Galaxy) =
        abs(this.x - other.x) + abs(this.y - other.y)
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
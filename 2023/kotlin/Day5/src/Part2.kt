fun main() {
    // Split input file into "sections"
    val input = getResourceAsText("/input.txt")?.split("\r\n\r\n")!!
    // First line is seed pairs; drop the "seeds:" text, then make a list as pairs
    val seedRanges = input[0].split(" ").drop(1).map { it.toLong() }
        .chunked(2) { (start, length) -> Range(start, start + length - 1) }
    // Remaining sections (after the first) are mappings
    val maps = input.drop(1).map { section ->
        val mapInput = section.split("\r\n")
        // Name the section based on what it is "from" to what it is "to"
        val (from, to) = mapInput[0].split("-to-", " map:")
        // Remaining lines (after the first of this section) are tuples
        val mappings = mapInput.drop(1).map { line ->
            val (d, s, l) = line.split(" ")
            RangeMapping(d.toLong(), s.toLong(), l.toLong())
        }
        DividingMapping(from, to, mappings)
    }
    seedRanges.minOf { range ->
        var ranges = listOf(range) // Start with a list of 1, but expect it to grow as it transforms
        var map = maps.first { it.from == "seed" } // Start with the "seed" mapping
        while(map.to != "location") {
            ranges = map.transform(ranges) // Apply transformations to all ranges
            map = maps.first { it.from == map.to } // Go to the next mapping pointed to by this one
        }
        map.transform(ranges).also { // One last transform
            println("$range -> $it") // Debug
        }.minOf { it.start } // Reduce the list of ranges to the lowest result; that's all we care about
    }.also {
        println("Lowest location number of all seeds: $it")
    }
}

data class Range(
    val start: Long,
    val end: Long,
    val delayedApply: Long? = null // We can't apply right away, otherwise each section row applies transformations
) {
    // Allow adding a number to this range, for convenience
    operator fun plus(increment: Long) =
        Range(start + increment, end + increment)

    // For debugging
    override fun toString() =
        "Range(start=$start, end=$end)"
}

data class DividingMapping(
    val from: String,
    val to: String,
    val rangeMappings: List<RangeMapping>
) {
    /** Converts the specified ranges into subRanges and apply transformations to subRanges that
     *  are contained within this mapping's rangeMappings. */
    fun transform(ranges: List<Range>) = ranges.flatMap { range ->
        // We start with just a range, but can continuously break it apart on rangeMapping boundaries
        rangeMappings.fold(listOf(range)) { acc: List<Range>, mapping -> acc.flatMap { subRange ->
            when {
                // If this subRange is outside the mapping, pass it through
                subRange.start > mapping.end() || subRange.end < mapping.source -> listOf(subRange)
                else -> buildList {
                    if(subRange.start < mapping.source)
                        add(Range(subRange.start, mapping.source - 1))
                    if(subRange.end > mapping.end())
                        add(Range(mapping.end() + 1, subRange.end))
                    // Do the actual transformation here, but delay it to prevent the next mapping from seeing it
                    add(Range(
                        Math.max(subRange.start, mapping.source),
                        Math.min(subRange.end, mapping.end()),
                        mapping.dest - mapping.source
                    ))
                }
            }
        }}
    }.map { range -> range.delayedApply?.let { range + it } ?: range } // Apply any delayed ranges
}

// Convenience to make code a little more readable
fun RangeMapping.end() =
    source + length - 1

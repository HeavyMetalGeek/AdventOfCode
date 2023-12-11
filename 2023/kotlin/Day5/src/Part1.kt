fun main() {
    // Split input file into "sections"
    val input = getResourceAsText("/input.txt")?.split("\r\n\r\n")!!
    // First line is seed numbers, but drop "seeds:" text
    val seeds = input[0].split(" ").drop(1).map { it.toLong() }
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
        Mapping(from, to, mappings)
    }
    // Debug for fun
    println(seeds)
    println(maps)
    seeds.minOf { seedId ->
        var num = seedId
        var map = maps.first { it.from == "seed" } // Start with the "seed" mapping
        while(map.to != "location") {
            num = map.transform(num) // Transform numbers if it is in the mapping
            map = maps.first { it.from == map.to } // Go to the next mapping pointed to by this one
        }
        map.transform(num).also { // One last transform
            println("$seedId -> $it") // Debug
        }
    }.also {
        println("Lowest location number of all seeds: $it")
    }
}

data class Mapping(
    val from: String,
    val to: String,
    val rangeMappings: List<RangeMapping>
) {
    fun transform(num: Long) =
        // If there's a mapping that can contain this number, transform it, otherwise pass it through
        rangeMappings.firstOrNull { num >= it.source && num < it.source + it.length }
            ?.let { num - it.source + it.dest }
            ?: num
}

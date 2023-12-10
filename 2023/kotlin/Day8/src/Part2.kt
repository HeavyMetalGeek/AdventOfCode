fun main() {
    getResourceAsText("/input.txt")?.lines()?.let { lines ->
        val directions = lines.first().toCharArray() // First line is e.g. LRLRLR
        var maps = parseMap(lines.drop(2)) // Rest are node definitions
            .filterValues { node -> node.name.endsWith("A") }
            .values.toList()
        maps.map {
            var node = it
            var steps = 0
            //val loop: MutableSet<String> = mutableSetOf()
            while(!node.name.endsWith("Z")) {
            /*while(true) {
                val t = "${steps % directions.size}$node"
                if(loop.contains(t))
                    break
                loop.add(t)*/
                node = when (directions[steps++ % directions.size]) {
                    'L' -> node.left!!
                    else -> node.right!!
                }
            }
            steps.toLong()
        }.also { stepList ->
            println(stepList)
            /* The commented code above serves as verification that A -> Z always loops.
            // If this were not the case, the following would not work.
            // From here, simply find the Least Common Multiple to save time. */
            val maxLcm = stepList.reduce { acc, multiple -> acc * multiple  }
            val gf = stepList.max()
            var lcm = gf
            while(lcm <= maxLcm) {
                if(stepList.all { lcm % it == 0L })
                    break
                lcm += gf
            }
            println("Got there in $lcm steps")
        }

        /* The following is the brute force method, but fails if steps > Int.MAX_VALUE.
        // That could easily be fixed, but if we're getting to that point, it is too slow. */
        /*var steps = 0
        while(!maps.all { it.name.endsWith("Z") }) {
            val nextDirection = directions[steps++ % directions.size]
            maps = maps.map {
                when(nextDirection) {
                    'L' -> it.left!!
                    else -> it.right!!
                }
            }
        }
        println("Got there in $steps steps")*/
    }
}

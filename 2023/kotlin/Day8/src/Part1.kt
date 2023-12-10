fun main() {
    getResourceAsText("/input.txt")?.lines()?.let { lines ->
        val directions = lines.first().toCharArray() // First line is e.g. LRLRLR
        var map = parseMap(lines.drop(2))["AAA"]!! // Rest are node definitions
        var steps = 0
        while(map.name != "ZZZ") {
            map = when(directions[steps++ % directions.size]) {
                'L' -> map.left!!
                else -> map.right!!
            }
        }
        println("Got there in $steps steps")
    }
}

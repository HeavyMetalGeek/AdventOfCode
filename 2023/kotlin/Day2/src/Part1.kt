fun main() {
    val requirement = mapOf("red" to 12, "green" to 13, "blue" to 14)
    getResourceAsText("/input.txt")?.lines()?.sumOf {
        val (gameId, mostCubes) = parseCubes(it)
        // Once we know the most cubes per game, filter out the ones that are impossible
        gameId.takeIf { requirement.all { (k, v) -> v >= (mostCubes[k] ?: 0) } } ?: 0
    }?.also {
        println("Sum of all winning game IDs: $it")
    }
}

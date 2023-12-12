fun main() {
    getResourceAsText("/input.txt")?.lines()?.sumOf {
        val (_, results) = parseCubes(it)
        // Just multiply the number of cubes with each other
        results.values.reduce { acc, cubes -> acc * cubes }
    }?.also {
        println("The sum of all game powers is: $it")
    }
}

fun main() {
    getResourceAsText("input.txt")?.lines()?.sumOf { line ->
        val parts = line.split(" ")
        val springs = Array(5) { parts[0] }.joinToString("?") // Repeats 5 times, separated by ?
        val splits = parts[1].split(",").map { it.toInt() }
            .let { list -> (1..5).flatMap { list } } // Repeats the list 5 times
        memoizationMap.clear() // Probably should've used a wrapper class
        StepData().bruteSolve(springs.toCharArray(), splits).also {
            // println("$springs $splits = $it") // Debug
        }
    }?.also {
        println("Total sum of possible arrangements after unfolding five times: $it")
    }
}

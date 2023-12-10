fun main() {
    getResourceAsText("/input.txt")?.lines()?.sumOf { line ->
        var row = line.split(" ").map { it.toInt() }.toIntArray()
        // Imagine a stack of rows to hold the final result
        val stack: MutableList<Int> = mutableListOf()
        while(row.any { it != 0 }) {
            stack.add(row.last())
            // Calculate differences as a new row
            row = IntArray(row.size - 1) { i ->
                row[i + 1] - row[i]
            }
        }
        // Optimization: turns out sum is the same as incrementing by difference
        stack.sum()
    }?.also {
        println("Extrapolated sum: $it")
    }
}

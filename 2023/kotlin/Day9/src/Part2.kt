fun main() {
    getResourceAsText("/input.txt")?.lines()?.sumOf { line ->
        var row = line.split(" ").map { it.toInt() }.toIntArray()
        // Imagine a stack of rows to hold the final result
        val stack: MutableList<Int> = mutableListOf()
        while(row.any { it != 0 }) {
            stack.add(row.first())
            // Calculate differences as a new row
            row = IntArray(row.size - 1) { i ->
                row[i + 1] - row[i]
            }
        }
        // reduceRight works right to left replacing each item with the operation on its right
        // so [10, 3, 0, 2] -> [10, 3, -2] -> [10, 5] -> 5
        stack.reduceRight { prev, acc -> prev - acc }
    }?.also {
        println("Extrapolated sum: $it")
    }
}

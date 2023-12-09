fun main() {
    getResourceAsText("/input.txt")?.lines()?.sumOf { line ->
        val base = line.split(" ").map { it.toInt() }.toIntArray()
        val stack: MutableList<Int> = mutableListOf()
        var row = base
        while(!row.all { it == 0 }) {
            stack.add(row.first())
            val newRow = IntArray(row.size - 1)
            for(i in newRow.indices) {
                newRow[i] = row[i + 1] - row[i]
            }
            row = newRow
        }
        stack.reverse()
        for(i in 1..<stack.size) {
            stack[i] = stack[i] - stack[i - 1]
        }
        stack.last()
    }?.also {
        println("Extrapolated sum: $it")
    }
}

fun main() {
    getResourceAsText("/input.txt")?.lines()?.map { it.toCharArray() }?.run {
        rotate().sumOf { line ->
            var freeSpace = 0
            line.withIndex().sumOf { (x, c) -> when(c) {
                'O' -> size - freeSpace++
                '#' -> 0.also { freeSpace = x + 1 }
                else -> 0
            } }
        }
    }?.also {
        println("Total load on north column is $it")
    }
}

fun List<CharArray>.rotate() = List(this[0].size) { y ->
    CharArray(size) { x -> this[x][y] }
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()

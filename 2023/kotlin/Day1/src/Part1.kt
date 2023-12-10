fun main() {
    getResourceAsText("/input.txt")?.lines()?.sumOf { line ->
        val array = line.toCharArray()
        val first = array.first { it.isDigit() }
        val last = array.last { it.isDigit() }
        (first - '0') * 10 + (last - '0')
    }?.also {
        println("Sum of calibration values: $it")
    }
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
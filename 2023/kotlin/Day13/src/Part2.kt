fun main() {
    getResourceAsText("/input.txt")?.split("\n\n", "\r\n\r\n")?.sumOf { section ->
        // Tries to find horizontal reflection; if it can't, tries vertical reflection
        section.lines().map { it.toCharArray() }.run {
            findReflection(1)?.let { 100 * (it + 1) } // Horizontal; 100 points per row
                ?: rotate().findReflection(1)?.plus(1) // Vertical; 1 point per row
                ?: error("Contract of expectation vitiated")
        }
    }?.also {
        println(it)
    }
}

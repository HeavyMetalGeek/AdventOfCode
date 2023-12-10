fun main() {
    getResourceAsText("/input.txt")?.lines()?.sumOf { line ->
        val array = line.toCharArray()
        val firstR = array.findNumber()
        val lastR = array.findNumber(true)
        firstR * 10 + lastR
    }?.also {
        println("Sum of calibration values: $it")
    }
}

/** Far less readable than a regex solution, but far more performant */
fun CharArray.findNumber(reversed: Boolean = false): Int {
    // Technically, zero wasn't in the rules and doesn't appear in the input, but needed for math to work out
    val numbers = listOf("zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine")
        .map { it.toCharArray() }
    val ids = if(!reversed) indices else indices.reversed()
    for(i in ids) {
        if(this[i].isDigit())
            return this[i] - '0'
        // Could be optimized further using buckets, but that would increase complexity and LoC
        numbers.forEachIndexed { result, num -> run {
            num.forEachIndexed { k, charInNum ->
                if(i + k >= size || this[i + k] != charInNum)
                    return@run // Ugly way to do a "continue" in a Kotlin forEach; needed for lambda
            }
            return result
        }}
    }
    error("Catastrophic meltdown imminent")
}

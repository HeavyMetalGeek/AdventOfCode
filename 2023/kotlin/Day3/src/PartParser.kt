fun parseParts(inputFile: String) = run {
    val numList: MutableList<Array<Num?>> = mutableListOf()
    val regex = "\\d+".toRegex()
    getResourceAsText(inputFile)?.lines()?.map { line ->
        // Store number references - it will only be included in the set below once
        val array = Array<Num?>(line.length) { null }.also { numList.add(it) }
        // Each position of the number contains a reference to the same instance
        regex.findAll(line).forEach { match ->
            val newNum = Num(match.value.toInt())
            match.range.forEach { x -> array[x] = newNum }
        }
        line.toCharArray()
    }?.let { numList to it } ?: error("KABOOM")
}

// Must *not* be a data class.  We need equals() to be an object ref compare, not value compare.
class Num(val value: Int)

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()

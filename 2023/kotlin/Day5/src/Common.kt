data class RangeMapping(
    val dest: Long,
    val source: Long,
    val length: Long
)

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
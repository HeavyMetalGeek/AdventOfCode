fun main() {
    val requirement = mapOf("red" to 12, "green" to 13, "blue" to 14)
    getResourceAsText("/input.txt")?.lines()?.sumOf {
        val (game, results) = it.split(": ")
        val mostCubes = results.split("; ").map {
            it.split(", ").associate {
                val (num, color) = it.split(" ")
                color to num.toInt()
            }
        }.let {
            mutableMapOf<String, Int>().apply {
                it.forEach {
                    it.forEach { (k, v) -> merge(k, v) { oldVal, newVal -> Math.max(oldVal, newVal) } }
                }
            }
        }
        game.split(" ")[1].toInt()
            .takeIf { requirement.all { (k, v) -> v >= (mostCubes[k] ?: 0) } } ?: 0
    }?.also {
        println(it)
    }
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
fun main() {
    getResourceAsText("/input.txt")?.split(",")?.sumOf {
        it.hash()
    }?.also {
        println("Sum of hashes: $it")
    }
}

/** Computes a hash by adding the ascii code to the accumulated value, multiplying, and wrapping around 256. */
fun String.hash() =
    fold(0) { acc, c -> ((acc + c.code) * 17) % 256 }

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()

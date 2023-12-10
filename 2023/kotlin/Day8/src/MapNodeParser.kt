/** Convert input text into an interconnected graph */
fun parseMap(input: List<String>): Map<String, MapNode> {
    val regex = Regex("(\\w+) = \\((\\w+), (\\w+)\\)")
    val cache: MutableMap<String, MapNode> = mutableMapOf()
    return input.associate { line ->
        val match = regex.find(line)!!
        val (name, left, right) = match.destructured
        val node = cache[name] ?: MapNode(name).also { cache[name] = it }
        val leftNode = cache[left] ?: MapNode(left).also { cache[left] = it }
        val rightNode = cache[right] ?: MapNode(right).also { cache[right] = it }
        node.left = leftNode
        node.right = rightNode
        name to node
    }
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
fun main() {
    // Can't make this a const; destructuring only works on local variables :/
    val (n, s, e, w) = listOf(0x1, 0x2, 0x4, 0x8)
    getResourceAsText("/input.txt")?.lines()?.map { it.toCharArray() }?.run {
        // No optimizations here; brute force every edge ray and find the largest of them
        listOf(
            indices.map { y -> energizeTilesFromRay(Ray(e, 0, y)) },
            indices.map { y -> energizeTilesFromRay(Ray(w, this[0].size - 1, y)) },
            this[0].indices.map { x -> energizeTilesFromRay(Ray(s, x, 0)) },
            this[0].indices.map { x -> energizeTilesFromRay(Ray(n, x, size - 1)) },
        ).flatten().max()
    }?.also {
        println("Most energized tiles: $it")
    }
}
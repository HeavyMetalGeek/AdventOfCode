fun main() {
    // Can't make this a const; destructuring only works on local variables :/
    val (n, s, e, w) = listOf(0x1, 0x2, 0x4, 0x8)
    getResourceAsText("/input.txt")?.lines()?.map { it.toCharArray() }?.run {
        // Start a rightward-moving ray from the top-left corner and return the number of energized tiles
        energizeTilesFromRay(Ray(e, 0, 0))
    }?.also {
        println("Energized tiles: $it")
    }
}

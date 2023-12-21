fun main() {
    getResourceAsText("/input.txt")?.lines()?.map {
        val (dir, dist, color) = it.split(" ")
        Instruction(Dir.valueOf(dir), dist.toInt(), color.trim('(', ')'))
    }?.let { instructions ->
        var pos = Pos(0, 0)
        // Move the position by the edge distance, then return the total area (inner + peri)
        instructions.map {
            pos += it.dir * it.dist
            Trench(pos, 1, it.color)
        }.area() + instructions.perimeter()
    }?.also {
        println("Area: $it")
    }
}

/** Uses an optimized Shoelace formula I found to find inner area. */
fun List<Trench>.area() = (1..size).sumOf { i ->
    this[i % size].pos.x * (this[(i + 1) % size].pos.y - this[i - 1].pos.y)
}.let { Math.abs(it) / 2 }

/** Compensate for the above area by adding two sides of the perimeter. Assuming a closed loop,
 * we can assume the area above always only considers the top and left parts of any rectangle,
 * meaning we have to re-add the bottom and right edges (+1 to close the loop.) */
fun List<Instruction>.perimeter() = sumOf { it.dist }.let { it / 2 + 1 }

data class Pos(val x: Long = 0, val y: Long = 0) {
    operator fun plus(other: Pos) = Pos(x + other.x, y + other.y)
}
enum class Dir(val x: Long, val y: Long) {
    U(0, -1),
    D(0, 1),
    L(-1, 0),
    R(1, 0);
    operator fun times(multiplier: Int) = Pos(x * multiplier, y * multiplier)
}
data class Instruction(val dir: Dir, val dist: Int, val color: String? = null)
data class Trench(val pos: Pos, var depth: Int, var color: String? = null)

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
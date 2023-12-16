/** With a grid of mirrors, follow each ray until it travels out of bounds or intersects a known path. */
fun List<CharArray>.energizeTilesFromRay(initialRay: Ray): Int {
    val (n, s, e, w) = listOf(0x1, 0x2, 0x4, 0x8)
    val forwardReflect = mapOf(e to n, s to w, n to e, w to s) // '/'
    val backReflect = mapOf(e to s, n to w, s to e, w to n) // '\'
    // The lightGrid tracks the direction a ray went for each tile using a bit field
    val lightGrid = Array(size) { Array(this[0].size) { BitField(0) } }
    // Didn't need to be a stack, I wanted O(1) add/remove
    val stack = ArrayDeque(listOf(initialRay))
    // Works similar to maze solving algorithms; start with a node and follow it until it is no longer re-added.
    // Since rays can fork at mirrors, having a stack of nodes allows us to finish traversal of one ray, then
    // come back for forked rays.
    while(stack.size > 0) {
        val ray = stack.removeFirst()
        lightGrid[ray.y][ray.x] += ray.dir // Bitwise | (Kotlin doesn't support this, so I made my own)
        when(this[ray.y][ray.x]) {
            '/' -> ray.dir = forwardReflect[ray.dir]!!
            '\\' -> ray.dir = backReflect[ray.dir]!!
            '-' -> if(ray.dir == n || ray.dir == s) {
                ray.dir = w; stack.add(Ray(e, ray.x, ray.y)) // Make this ray go West, add a new ray going East
            }
            '|' -> if(ray.dir == w || ray.dir == e) {
                ray.dir = n; stack.add(Ray(s, ray.x, ray.y)) // Same as above, except North and South
            }
        }
        // Move the ray by its direction. Remove if outside the grid or lands on a previously traversed spot.
        ray.move()
        if(!lightGrid.outOfBounds(ray.x, ray.y) && lightGrid[ray.y][ray.x][ray.dir] == 0)
            stack.addFirst(ray)
    }
    // Debugging
    /*lightGrid.forEachIndexed { y, row ->
        println(row.mapIndexed { x, bit ->
            this[y][x].takeIf { it != '.' } ?: '#'.takeIf { bit.value > 0 } ?: '.'
            //'#'.takeIf { bit.value > 0 } ?: '.'
        }.let { String(it.toCharArray()) })
    }*/
    return lightGrid.sumOf { row -> row.count { it.value > 0 } }
}

fun <T> Array<Array<T>>.outOfBounds(x: Int, y: Int) =
    x < 0 || y < 0 || x >= this[0].size || y >= size

/** Mutable.  Could've been immutable where move() returned a new instance, but mutable is a little faster. */
data class Ray(var dir: Int, var x: Int, var y: Int) {
    fun move() { when {
        (dir and 1) != 0 -> y-- // N
        (dir and 2) != 0 -> y++ // S
        (dir and 4) != 0 -> x++ // E
        (dir and 8) != 0 -> x-- // W
    } }
}

/** Ugh, Kotlin bit support is sad. Very sad.  This class is necessary because of a lack of bit support. */
class BitField(var value: Int) {
    operator fun plusAssign(bit: Int) { value = value or bit } // Bitwise | (OR) assignment
    operator fun get(index: Int) = value and index // Bitwise & (AND)
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()

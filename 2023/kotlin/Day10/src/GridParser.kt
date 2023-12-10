data class GridWrapper(
    val start: Position,
    val grid: List<CharArray>
) {
    companion object {
        fun parseGrid(inputFile: String) = run {
            var start: Position? = null
            val grid = getResourceAsText(inputFile)?.lines()?.mapIndexed { y, line ->
                line.indexOf("S").takeIf { it != -1 }?.let { start = Position(it, y) }
                line.toCharArray()
            }
            GridWrapper(start!!, grid!!) // Ew
        }
    }

    fun findPipe(nodeTraversalFunc: (DirectedPosition) -> Unit) {
        val row = grid[start.y]
        val direction = buildList {
            // Find adjacent in-network pipes to S
            if(start.x > 0 && row[start.x - 1] in listOf('-', 'F', 'L')) add(Direction.WEST)
            if(start.x + 1 < row.size && row[start.x + 1] in listOf('-', '7', 'J')) add(Direction.EAST)
            if(start.y > 0 && grid[start.y - 1][start.x] in listOf('|', 'F', '7')) add(Direction.NORTH)
            if(start.y + 1 < grid.size && grid[start.y + 1][start.x] in listOf('|', 'L', 'J')) add(Direction.SOUTH)
        }.also {
            // Transform S into its equivalent pipe piece (only needed for part 2)
            grid[start.y][start.x] = when {
                it.contains(Direction.NORTH) -> when {
                    it.contains(Direction.WEST) -> 'J'
                    it.contains(Direction.EAST) -> 'L'
                    else -> '|'
                }
                it.contains(Direction.SOUTH) -> when {
                    it.contains(Direction.WEST) -> '7'
                    else -> 'F'
                }
                else -> '-'
            }
        }.first() // Pick an arbitrary direction
        val node = DirectedPosition(direction, start.x, start.y)
        do {
            nodeTraversalFunc(node) // Magic
            node.follow()
            // Change direction based on corner pipes
            node.direction = when(grid[node.y][node.x]) {
                '7' -> if(node.direction == Direction.EAST) Direction.SOUTH else Direction.WEST
                'J' -> if(node.direction == Direction.EAST) Direction.NORTH else Direction.WEST
                'F' -> if(node.direction == Direction.WEST) Direction.SOUTH else Direction.EAST
                'L' -> if(node.direction == Direction.WEST) Direction.NORTH else Direction.EAST
                else -> node.direction // Maintain current heading (applies to '-' and '|')
            }
        } while(node.x != start.x || node.y != start.y)
    }
}


enum class Direction {
    NORTH, SOUTH, EAST, WEST
}

data class Position(val x: Int, val y: Int)

data class DirectedPosition(
    var direction: Direction,
    var x: Int,
    var y: Int
) {

    fun follow() = when(direction) {
        Direction.NORTH -> y -= 1
        Direction.SOUTH -> y += 1
        Direction.EAST -> x += 1
        Direction.WEST -> x -= 1
    }
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
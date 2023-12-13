fun main() {
    GridWrapper.parseGrid("/input.txt").run {
        val pipeMap = MutableList(grid.size) { BooleanArray(grid[0].size) }
        findPipe {
            pipeMap[it.y][it.x] = true
        }
        var mode = '|'
        grid.mapIndexed { y, line ->
            var a = false
            // A very strange-looking raycast algorithm; intersections are |, FJ, and L7
            line.filterIndexed { x, c ->
                when(pipeMap[y][x]) {
                    true -> false.also { when(mode) {
                        '|' -> when(c) {
                            '|' -> a = !a
                            'F', 'L' -> mode = c
                        }
                        'F' -> when(c) {
                            '|', 'J' -> { a = !a; mode = '|'}
                            '7' -> mode = '|'
                        }
                        'L' -> when(c) {
                            '|', '7' -> { a = !a; mode = '|' }
                            'J' -> mode = '|'
                        }
                    }}
                    else -> a
                }
            }.count().also {
                // Cool debug view of pipe network
                println(line.mapIndexed { x, _ -> grid[y][x].takeIf{ pipeMap[y][x] } ?: " " }.toString().replace(", ", "") + " $it")
            }
        }.sum().also {
            println("$it tiles are enclosed in the pipe network")
        }
    }
}

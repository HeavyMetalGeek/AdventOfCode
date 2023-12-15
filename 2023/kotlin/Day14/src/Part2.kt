fun main() {
    getResourceAsText("/input.txt")?.lines()?.map { it.toCharArray() }?.run {
        repeat(1000) { step ->
            // North
            (0..<size).forEach { x ->
                var swapTo = 0
                (0..<size).forEach { y ->
                    when(this[y][x]) {
                        'O' -> { val c = this[y][x]; this[y][x] = this[swapTo][x]; this[swapTo][x] = c; swapTo++ }
                        '#' -> swapTo = y + 1
                    }
                }
            }
            // West
            (0..<size).forEach { y ->
                var swapTo = 0
                (0..<size).forEach { x ->
                    when(this[y][x]) {
                        'O' -> { val c = this[y][x]; this[y][x] = this[y][swapTo]; this[y][swapTo] = c; swapTo++ }
                        '#' -> swapTo = x + 1
                    }
                }
            }
            // South
            (0..<size).forEach { x ->
                var swapTo = size - 1
                (0..<size).reversed().forEach { y ->
                    when(this[y][x]) {
                        'O' -> { val c = this[y][x]; this[y][x] = this[swapTo][x]; this[swapTo][x] = c; swapTo-- }
                        '#' -> swapTo = y - 1
                    }
                }
            }
            // East
            (0..<size).forEach { y ->
                var swapTo = size - 1
                (0..<size).reversed().forEach { x ->
                    when(this[y][x]) {
                        'O' -> { val c = this[y][x]; this[y][x] = this[y][swapTo]; this[y][swapTo] = c; swapTo-- }
                        '#' -> swapTo = x - 1
                    }
                }
            }
            // Count
            withIndex().sumOf { (y, line) ->
                line.count { it == 'O' } * (size - y)
            }.also { println("$step: Load $it") }
        }
    }?.also {
        println("Total load on north column is $it")
    }
}

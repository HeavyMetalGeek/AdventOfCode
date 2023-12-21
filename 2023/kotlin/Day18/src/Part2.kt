fun main() {
    val directions = arrayOf(Dir.R, Dir.D, Dir.L, Dir.U)
    getResourceAsText("/input.txt")?.lines()?.map {
        val (_, _, color) = it.split(" ")
        // Dist/Dir derived from "color", dist is a hex string, dir is limited to [0-3]
        val (dist, dir) = color.trim('(', ')', '#').splitAt(5)
        Instruction(directions[dir.toInt()], dist.toInt(16))
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

fun String.splitAt(index: Int) = let { take(index) to substring(index) }

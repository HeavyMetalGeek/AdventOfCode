fun main() {
    val (numList, charGrid) = parseParts("/input.txt")
    val products = mutableListOf<Int>()
    // The legendary quadruple for loop!
    // Adding all 9 neighbors around each symbol as references to a set automatically eliminates duplicates.
    charGrid.forEachIndexed { y, chars ->
        chars.forEachIndexed { x, c -> if(c == '*') {
            val gearSet = mutableSetOf<Num?>()
            for(yi in y - 1 .. y + 1)
                for(xi in x - 1 .. x + 1)
                    gearSet.add(numList[yi][xi])
            // Only a gear if it has 2 neighbors; multiply the two together
            gearSet.filterNotNull().takeIf { it.size == 2 }
                ?.map { it.value }?.reduce { acc, i -> acc * i }
                ?.also { products.add(it) }
        }}
    }
    products.sum().also {
        println("Sum of all gear ratio products: $it")
    }
}

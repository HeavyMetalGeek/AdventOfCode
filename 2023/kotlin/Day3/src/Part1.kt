fun main() {
    val (numList, charGrid) = parseParts("/input.txt")
    val resultSet = mutableSetOf<Num?>()
    // The legendary quadruple for loop!
    // Adding all 9 neighbors around each symbol as references to a set automatically eliminates duplicates.
    charGrid.forEachIndexed { y, chars ->
        chars.forEachIndexed { x, c -> if(!c.isDigit() && c != '.') {
            for(yi in y - 1 .. y + 1)
                for(xi in x - 1 .. x + 1)
                    resultSet.add(numList[yi][xi])
        }}
    }
    resultSet.filterNotNull().sumOf { it.value }.also {
        println("Sum of all part numbers: $it")
    }
}

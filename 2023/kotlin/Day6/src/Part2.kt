fun main() {
    getResourceAsText("input.txt")?.lines()?.map { line ->
        // Remove all non-digit characters from each row and return it as a number
        line.replace(" ", "").replace("\\D".toRegex(), "").toLong()
    }?.let { (time, distance) ->
        // Not very Kotlin, but way more efficient than the Part 1 implementation
        var current = 0L
        var end = time / 2
        // Use a divide-and-conquer strategy for finding the number that *can't* beat the distance
        while (current != end) {
            val half = (current + end + 1) / 2 // Force half to be right-sided
            when (half * (time - half) > distance) {
                true -> end = half - 1 // Don't reconsider winning scenarios
                false -> current = half // But always reconsider losing scenarios
            }
        }
        time - 2 * current - 1 // "current" is the first failure, so everything in between wins
    }?.also {
        println("Ways to win: $it")
    }
}

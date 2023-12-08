fun main() {
    val strength = "AKQJT98765432"

    getResourceAsText("/input.txt")?.let { input ->
        parseHand(strength, input)
                .sortedWith(compareBy(Hand::type1).then(naturalOrder()))
                .printWinnings()
    } ?: error("input.txt not found... Sadge")
}

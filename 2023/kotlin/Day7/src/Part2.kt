fun main() {
    val strength = "AKQT98765432J"

    getResourceAsText("/input.txt")?.let { input ->
        parseHand(strength, input)
                .sortedWith(compareBy(Hand::type2).then(naturalOrder()))
                .printWinnings()
    } ?: error("input.txt not found... Sadge")
}

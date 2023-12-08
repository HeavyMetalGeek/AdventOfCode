data class Hand(
    val cardsDebug: String, // Original string, for debugging
    val cards: List<Int>, // Optimized
    val bid: Int
) : Comparable<Hand> {
    companion object {
        // Ordered by strength
        enum class Type {
            FiveOfAKind,
            FourOfAKind,
            FullHouse,
            ThreeOfAKind,
            TwoPair,
            OnePair,
            HighCard
        }
    }

    /** Get the type for Part 1 (optimization included for Part 2) */
    fun type1(optimization: Map<Int, Int>? = null): Type {
        val dupes = optimization ?: cards.groupingBy { it }.eachCount()
        // dupes.size refers to number of unique card types
        // dupes.containsValue returns true if any card has "N" of a kind
        return when(dupes.size) {
            1 -> Type.FiveOfAKind
            2 -> if(dupes.containsValue(4)) Type.FourOfAKind else Type.FullHouse
            3 -> if(dupes.containsValue(3)) Type.ThreeOfAKind else Type.TwoPair
            4 -> Type.OnePair
            else -> Type.HighCard
        }
    }

    /** Get the type for Part 2 */
    fun type2(): Type {
        val dupes = cards.groupingBy { it }.eachCount()
        val j = 12 // When J is the last in the rank, its index is always 12
        if(dupes[j] == null)
            return type1(dupes) // If no J is present, use original method
        return when(dupes.size) {
            1, 2 -> Type.FiveOfAKind
            3 -> when(dupes[j]) {
                2, 3 -> Type.FourOfAKind
                else -> if(dupes.containsValue(3)) Type.FourOfAKind else Type.FullHouse
            }
            4 -> Type.ThreeOfAKind
            else -> Type.OnePair
        }
    }

    /** Creates a "naturalOrder" for Hands */
    override fun compareTo(other: Hand): Int {
        cards.forEachIndexed { i, card ->
            // K > Q, for example
            val compare = card compareTo other.cards[i]
            if (compare != 0) // If strengths match, check the next card
                return compare
        }
        // Rules are undefined for this situation
        error("non-unique input detected")
    }

    /** For debugging */
    override fun toString() =
        "$cardsDebug=$bid"
}

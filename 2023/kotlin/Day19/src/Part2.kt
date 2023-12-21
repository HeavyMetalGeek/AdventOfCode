fun main() {
    getResourceAsText("/input.txt")?.split("\r\n\r\n", "\n\n")?.let { (workflows, _) ->
        // Workflows contain a set of rules that could link to other workflows
        workflows.lines().associate {
            it.split("{").let { (name, rules) ->
                name to Workflow(name, rules.trim('}'))
            }
        // We delay "parsing" the rules so that we can make a reference to the workflow
        }.apply { values.forEach { it.parse(this) } }.let {
            // Starting with the "in" workflow, reduce the set of xmas ratings for each rule
            it["in"]!!.reduce(listOf('x', 'm', 'a', 's').associateWith { listOf(1..4000) })
        }.also {
            println("Sum of products: $it")
        }
    }
}

/** Reduce the ranges for each rule, summing the results and passing unevaluated ranges to the
 * next rule. */
fun Workflow.reduce(ranges: Map<Char, List<IntRange>>): Long = run {
    var nextRanges = ranges
    rules.sumOf { rule -> rule.reduce(nextRanges).let { (s, r) -> nextRanges = r; s } }
}

/** If this rule points to a workflow, pass the range split by the category to it, otherwise
 * calculate the result from this rule.  A range split occurs when there is a category like 'x'
 * and if we say x: [1-4000] and the condition is < 1000, this rule will evaluate the x:[1-999]
 * portion and pass the [1000-4000] back up to the workflow to use with the next rule. */
fun BaseRule.reduce(ranges: Map<Char, List<IntRange>>) = when(this) {
    is Rule -> when(condition) {
        // If '>', we need to swap 'a' and 'b' so that the rhs is the one being evaluated
        '>' -> ranges[category]!!.split(rating + 1).let { (a, b) -> b to a }
        else -> ranges[category]!!.split(rating)
    }.let { (a, b) ->
        // Evaluate 'a' as if it replaced the category's range, then pass 'b' back to the workflow
        (ranges + (category to a)).let { dest?.reduce(it) ?: calculate(it) } to ranges + (category to b)
    }
    // The above applies only to rules that have a condition; below is just a BaseRule
    else -> (dest?.reduce(ranges) ?: calculate(ranges)) to ranges
}

/** If this rule terminates at 'R', return 0.  Otherwise, find the product of the ranges. */
fun BaseRule.calculate(ranges: Map<Char, List<IntRange>>) = if(!accept) 0L else
    ranges.values.map { l -> l.sumOf { it.last - it.first + 1 }.toLong() }.reduce { acc, i -> acc * i }

/** Splits at the specified index, left side excluding the given value. */
fun List<IntRange>.split(at: Int) = run {
    val middle = singleOrNull { it.first < at && it.last >= at }
        ?.let { it.first..<at to at..it.last }
    val left = filter { it.last < at }
    val right = filter { it.first >= at }
    middle?.let { (midLeft, midRight) ->
        left.plusElement(midLeft) to listOf(midRight) + right
    } ?: (left to right)
}

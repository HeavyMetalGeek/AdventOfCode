fun main() {
    getResourceAsText("/input.txt")?.split("\r\n\r\n", "\n\n")?.let { (workflows, parts) ->
        // Workflows contain a set of rules that could link to other workflows
        val parsedWorkflows = workflows.lines().associate {
            it.split("{").let { (name, rules) ->
                name to Workflow(name, rules.trim('}'))
            }
        // We delay "parsing" the rules so that we can make a reference to the workflow
        }.apply { values.forEach { it.parse(this) } }
        // These parts need to be sorted according to the rules in the workflows above
        val parsedParts = parts.lines().map { part ->
            Part(part.trim('{', '}').split(",").associate {
                val (category, rating) = it.split("=")
                category[0] to rating.toInt()
            })
        }
        // For each part, sort them always starting with the "in" workflow
        val startWorkflow = parsedWorkflows["in"]!!
        parsedParts.filter { startWorkflow.accept(it) }
    }?.also { acceptedParts ->
        // For each accepted part, sum their ratings, then sum the sums
        val sum = acceptedParts.sumOf { it.ratings.values.sum() }
        println("Sum of accepted part ratings: $sum")
    }
}

/** A workflow is just a named set of rules. Workflows may link to one another via these rules. */
data class Workflow(val name: String, val rulesToParse: String) {
    lateinit var rules: List<BaseRule>

    // Parse the rules; some link to other workflows with or without conditions and some just terminate
    fun parse(workflows: Map<String, Workflow>) {
        rules = rulesToParse.split(",").map { rule -> when(rule.contains(":")) {
            // If true, this rule has a condition and may go to a workflow or terminate
            true -> rule.split(":").let { (cond, dest) ->
                val destWorkflow = workflows[dest]
                Rule(cond[0], cond[1], cond.substring(2).toInt(), destWorkflow, dest == "A")
            }
            // If false, this rule just forwards to a workflow or terminates
            else -> BaseRule(workflows[rule], rule == "A")
        } }
    }

    // Evaluate a part by checking the rules. Only one rule needs to succeed in accepting it.
    fun accept(part: Part) = rules.firstNotNullOf { it.accept(part) }
}

data class Part(val ratings: Map<Char, Int>)

open class BaseRule(open val dest: Workflow?, open val accept: Boolean) {
    open fun accept(part: Part): Boolean? = dest?.accept(part) ?: accept
}
/** A Rule can terminate or forward like a BaseRule, but also has a condition to consider. */
data class Rule(val category: Char, val condition: Char, val rating: Int, override val dest: Workflow?, override val accept: Boolean = false)
    : BaseRule(dest, accept) {
    override fun accept(part: Part): Boolean? = when(condition) {
        '>' -> part.ratings[category]!! > rating
        else -> part.ratings[category]!! < rating
    }.takeIf { it }?.let { super.accept(part) }
}

/** Shenanigans to read from the "res" folder instead of hardcoded path */
fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()

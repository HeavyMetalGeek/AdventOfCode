fun main() {
    val boxes = Array<MutableMap<String, Int>>(256) { mutableMapOf() }
    getResourceAsText("/input.txt")?.split(",")?.forEach { step ->
        when(step.last()) {
            // If the last character is '-', remove that character and remove the lens from the box
            '-' -> step.trimEnd('-').run { boxes[hash()].remove(this) }
            // Otherwise, last character is probably a lens id. Parse it out and put it in the right box
            else -> step.split("=").let { (s, lens) -> boxes[s.hash()][s] = lens.toInt() }
        }
    }
    boxes.withIndex().sumOf { (boxId, box) ->
        // Compute the "focus power" by multiplying box id, lens position, and lens focal length
        box.values.withIndex().fold(0) { acc, (i, lens) -> acc + (boxId + 1) * (i + 1) * lens } as Int
    }.also {
        println("Focusing power is $it")
    }
}

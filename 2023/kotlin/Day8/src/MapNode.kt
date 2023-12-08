data class MapNode(
    val name: String,
    var left: MapNode? = null,
    var right: MapNode? = null
) {
    /** For debugging */
    override fun toString() = name
}
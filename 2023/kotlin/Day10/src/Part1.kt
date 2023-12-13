fun main() {
    var distance = 0
    GridWrapper.parseGrid("/input.txt").findPipe {
        distance++
    }
    println("Length of path: $distance, furthest point: ${distance / 2}")
}

package model.quantities.distance

enum class DistanceKeyword(private val symbol: String) : Distance {
    TOUCH("touch"),
    SELF("self");

    override fun toString(): String {
        return symbol
    }
}
package model.quantities.distance

enum class DistanceUnitLiteral(val symbol: String) : DistanceUnit {
    FOOT("ft"),
    MILE("mi"),
    SPACE("sp");

    override fun getDistanceUnitLiteral(): DistanceUnitLiteral {
        return this
    }

    override fun toString(): String {
        return symbol
    }
}
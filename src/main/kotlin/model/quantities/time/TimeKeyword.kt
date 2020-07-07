package model.quantities.time

enum class TimeKeyword(private val symbol: String) : Time {
    INSTANTANEOUS("instantaneous"),
    INDEFINITE("indefinite");

    override fun toString(): String {
        return symbol
    }
}

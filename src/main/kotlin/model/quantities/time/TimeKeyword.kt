package model.quantities.time

enum class TimeKeyword(private val symbol: String) : Time {
    INSTANTANEOUS("instantaneous"),
    INDEFINITE("indefinite"),
    NOW("now");

    override fun toString(): String {
        return symbol
    }
}

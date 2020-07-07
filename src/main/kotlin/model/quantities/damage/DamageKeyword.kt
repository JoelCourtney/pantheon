package model.quantities.damage

enum class DamageKeyword(private val symbol: String) : Damage {
    NONE("none");

    override fun toString(): String {
        return symbol
    }
}
package model.quantities.amounts

data class NumberLiteral(val n: Int) : Amount {
    constructor(s: String) : this(s.toInt())

    override fun toString(): String {
        return n.toString()
    }
}

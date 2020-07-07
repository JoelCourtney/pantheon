package model.quantities.amounts

data class Dice(val n: Int, val size: Int) : Amount {
    constructor(s1: String, s2: String) : this(s1.toInt(), s2.toInt())

    override fun toString(): String {
        return n.toString() + "d" + size.toString()
    }
}
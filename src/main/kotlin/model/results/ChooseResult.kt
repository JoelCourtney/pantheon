package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.*
import model.identity.Accessible
import model.identity.Identifier
import model.quantities.amounts.AmountBinaryOp

data class ChooseResult<T>(
    val choose: String,

    @JsonProperty("filter out")
    val filterOut: Identifier<Array<T>>? = null,

    @JsonProperty("filter except")
    val filterExcept: Identifier<Array<T>>? = null,

    @JsonProperty("choices")
    val numChoices: Int = 1,
    val unique: Boolean = false,

    val results: Array<Result>
): Result, Accessible {
    val chooseFrom: MutableList<T> = mutableListOf()
    val expandedResults: Array<Array<Result>> = Array(numChoices) { results.clone() }
    val values: MutableList<T?> = MutableList(3) { null }
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }

    override fun purge(): Boolean {
        TODO("Not yet implemented")
    }

    override fun get(id: String): Any {
        TODO("Not yet implemented")
    }

    override fun set(id: String, value: String) {
        TODO("Not yet implemented")
    }

    override fun modify(op: AmountBinaryOp, value: String) {
        TODO("Not yet implemented")
    }
}

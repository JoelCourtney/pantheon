package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.*
import model.identity.Accessible
import model.identity.Identifier
import model.quantities.amounts.AmountBinaryOp

data class ChooseResult<T>(
    val choose: String,

    @JsonProperty("filter out")
    val filterOut: Identifier<Array<String>>? = null,

    @JsonProperty("filter except")
    val filterExcept: Identifier<Array<String>>? = null,
    
    @JsonProperty("filter out list")
    val filterOutList: Array<String> = arrayOf(),
    
    @JsonProperty("filter except list")
    val filterExceptList: Array<String> = arrayOf(),

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

    override fun toString(): String {
        return "ChooseResult(choose='$choose', filterOut=$filterOut, filterExcept=$filterExcept, filterOutList=${filterOutList.contentToString()}, filterExceptList=${filterExceptList.contentToString()}, numChoices=$numChoices, unique=$unique, results=${results.contentToString()}, chooseFrom=$chooseFrom, expandedResults=${expandedResults.contentToString()}, values=$values)"
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ChooseResult<*>

        if (choose != other.choose) return false
        if (filterOut != other.filterOut) return false
        if (filterExcept != other.filterExcept) return false
        if (!filterOutList.contentEquals(other.filterOutList)) return false
        if (!filterExceptList.contentEquals(other.filterExceptList)) return false
        if (numChoices != other.numChoices) return false
        if (unique != other.unique) return false
        if (!results.contentEquals(other.results)) return false
        if (chooseFrom != other.chooseFrom) return false
        if (!expandedResults.contentDeepEquals(other.expandedResults)) return false
        if (values != other.values) return false

        return true
    }

    override fun hashCode(): Int {
        var result = choose.hashCode()
        result = 31 * result + (filterOut?.hashCode() ?: 0)
        result = 31 * result + (filterExcept?.hashCode() ?: 0)
        result = 31 * result + filterOutList.contentHashCode()
        result = 31 * result + filterExceptList.contentHashCode()
        result = 31 * result + numChoices
        result = 31 * result + unique.hashCode()
        result = 31 * result + results.contentHashCode()
        result = 31 * result + chooseFrom.hashCode()
        result = 31 * result + expandedResults.contentDeepHashCode()
        result = 31 * result + values.hashCode()
        return result
    }
}

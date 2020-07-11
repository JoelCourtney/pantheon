package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.gameObjects.Character
import model.access.Accessible
import model.access.Identifier
import model.quantities.amounts.AmountBinaryOp

data class ForEachResult<T>(
    @JsonProperty("for each")
    val forEach: String,

    @JsonProperty("filter out")
    val filterOut: Identifier<Array<String>>? = null,

    @JsonProperty("filter except")
    val filterExcept: Identifier<Array<String>>? = null,
    
    @JsonProperty("filter out list")
    val filterOutList: Array<String> = arrayOf(),
    
    @JsonProperty("filter except list")
    val filterExceptList: Array<String> = arrayOf(),

    val results: Array<Result>
): Result, Accessible {
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

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ForEachResult<*>

        if (forEach != other.forEach) return false
        if (filterOut != other.filterOut) return false
        if (filterExcept != other.filterExcept) return false
        if (!filterOutList.contentEquals(other.filterOutList)) return false
        if (!filterExceptList.contentEquals(other.filterExceptList)) return false
        if (!results.contentEquals(other.results)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = forEach.hashCode()
        result = 31 * result + (filterOut?.hashCode() ?: 0)
        result = 31 * result + (filterExcept?.hashCode() ?: 0)
        result = 31 * result + filterOutList.contentHashCode()
        result = 31 * result + filterExceptList.contentHashCode()
        result = 31 * result + results.contentHashCode()
        return result
    }

}

package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.quantities.Identifier

data class FileChoiceResult(
    @JsonProperty("file choice")
    val dir: String,
    
    @JsonProperty("filter out")
    val filterOut: Identifier? = null,
    
    @JsonProperty("filter except")
    val filterExcept: Identifier? = null,
    
    val results: Array<Result>
): Result {
    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }

    override fun purge(): Boolean {
        TODO("Not yet implemented")
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as FileChoiceResult

        if (dir != other.dir) return false
        if (filterOut != other.filterOut) return false
        if (filterExcept != other.filterExcept) return false
        if (!results.contentEquals(other.results)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = dir.hashCode()
        result = 31 * result + (filterOut?.hashCode() ?: 0)
        result = 31 * result + (filterExcept?.hashCode() ?: 0)
        result = 31 * result + results.contentHashCode()
        return result
    }

    override fun toString(): String {
        return "FileChoiceResult(dir='$dir', filterOut=$filterOut, filterExcept=$filterExcept, results=${results.contentToString()})"
    }
}

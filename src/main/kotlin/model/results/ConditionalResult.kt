package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.logic.Question

data class ConditionalResult(
    @JsonProperty("if")
    val ifTrue: Array<Question> = arrayOf(),
    @JsonProperty("if not")
    val ifFalse: Array<Question> = arrayOf(),

    @JsonProperty("success results")
    val successResults: Array<Result> = arrayOf(),
    @JsonProperty("failure results")
    val failureResults: Array<Result> = arrayOf()

) : Result {
    override fun purge(): Boolean {
        TODO("Not yet implemented")
    }

    override fun apply(c: Character) {
        TODO("Not yet implemented")
    }
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ConditionalResult

        if (!ifTrue.contentEquals(other.ifTrue)) return false
        if (!ifFalse.contentEquals(other.ifFalse)) return false
        if (!successResults.contentEquals(other.successResults)) return false
        if (!failureResults.contentEquals(other.failureResults)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = ifTrue.contentHashCode()
        result = 31 * result + ifFalse.contentHashCode()
        result = 31 * result + successResults.contentHashCode()
        result = 31 * result + failureResults.contentHashCode()
        return result
    }
}
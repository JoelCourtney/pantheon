package model.results

import com.fasterxml.jackson.annotation.JsonProperty
import model.Character
import model.quantities.amounts.Amount

data class SavingThrowResult(
    @JsonProperty("saving throw")
    val savingThrowType: SavingThrowType,
    val dc: Amount,
    @JsonProperty("fail results")
    val failResults: Array<Result> = arrayOf(),
    @JsonProperty("success results")
    val successResults: Array<Result> = arrayOf(),
    @JsonProperty("half damage on success")
    val halfDamageOnSuccess: Boolean = false
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

        other as SavingThrowResult

        if (savingThrowType != other.savingThrowType) return false
        if (dc != other.dc) return false
        if (!failResults.contentEquals(other.failResults)) return false
        if (!successResults.contentEquals(other.successResults)) return false
        if (halfDamageOnSuccess != other.halfDamageOnSuccess) return false

        return true
    }

    override fun hashCode(): Int {
        var result = savingThrowType.hashCode()
        result = 31 * result + dc.hashCode()
        result = 31 * result + failResults.contentHashCode()
        result = 31 * result + successResults.contentHashCode()
        result = 31 * result + halfDamageOnSuccess.hashCode()
        return result
    }
}

package model.modifications.results

import Engine
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.AmountDeserializer
import io.deserializers.SavingThrowTypeDeserializer
import model.gameObjects.Character
import model.access.Expression
import model.modifications.Modification
import model.modifications.effects.Effect
import model.quantities.amounts.Amount

data class SavingThrowResult(
        @JsonProperty("saving throw")
    @JsonDeserialize(using = SavingThrowTypeDeserializer::class)
    val savingThrowType: Expression<SavingThrowType>,
        @JsonDeserialize(using = AmountDeserializer::class)
    val dc: Expression<Amount>,
        @JsonProperty("fail results")
    val failResults: Array<Result> = arrayOf(),
        @JsonProperty("success results")
    val successResults: Array<Result> = arrayOf(),
        @JsonProperty("half damage on success")
    val halfDamageOnSuccess: Boolean = false
): Result {
    override fun applyResult(e: Engine): Effect? {
        TODO("Not yet implemented")
    }

    override fun purge(): Boolean {
        TODO("Not yet implemented")
    }
}

package model.modifications

import Engine
import com.fasterxml.jackson.annotation.JsonProperty
import model.access.Accessible
import model.access.Identifier
import model.gameObjects.Character
import model.modifications.effects.Effect
import model.quantities.amounts.AmountBinaryOp

data class ChoiceLogic<T>(
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

    val modifications: Array<Modification>
): Logic, Accessible {
    override fun applyEffect(e: Engine) {
        TODO("Not yet implemented")
    }

    override fun applyResult(e: Engine): Effect? {
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

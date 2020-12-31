package model.effects

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.ProficiencyEffectDeserializer
import model.gameObjects.Proficiency

@JsonDeserialize(using = ProficiencyEffectDeserializer::class)
data class ProficiencyEffect(
    val proficiency: Proficiency
): Effect() {
    override fun apply() {
        TODO("Not yet implemented")
    }

    override fun findDependencies(): List<String> {
        TODO("Not yet implemented")
    }

    override fun findEffected(): List<String> {
        TODO("Not yet implemented")
    }
}
package model.effects

import com.fasterxml.jackson.annotation.JsonProperty

data class FeatureEffect(
    @JsonProperty("feature")
    val name: String,
    val effects: List<Effect>,
    val description: String
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
package model.effects

import com.fasterxml.jackson.annotation.JsonProperty

data class SubclassEffect(
    @JsonProperty("subclass")
    val name: String
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
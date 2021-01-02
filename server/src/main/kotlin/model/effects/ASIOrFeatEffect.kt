package model.effects

import com.fasterxml.jackson.annotation.JsonIgnoreProperties

@JsonIgnoreProperties("asi or feat")
object ASIOrFeatEffect: Effect() {
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
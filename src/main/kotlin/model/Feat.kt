package model

import model.identity.Evaluated
import model.results.Result

data class Feat(
    val name: String,
    val description: String,
    val results: Array<Result>
): Evaluated<Feat> {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Feat

        if (name != other.name) return false
        if (description != other.description) return false
        if (!results.contentEquals(other.results)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + description.hashCode()
        result = 31 * result + results.contentHashCode()
        return result
    }

}

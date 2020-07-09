package model.races

import model.results.Result

data class VariantRace(
    val variantName: String,
    val variantOf: Race,
    val extraTraits: Array<Result>,
    val extraDescription: String,
    override val chooseable: Boolean
) : Race(variantOf) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as VariantRace

        if (variantName != other.variantName) return false
        if (variantOf != other.variantOf) return false
        if (!traits.contentEquals(other.traits)) return false
        if (extraDescription != other.extraDescription) return false
        if (chooseable != other.chooseable) return false

        return true
    }

    override fun hashCode(): Int {
        var result = variantName.hashCode()
        result = 31 * result + variantOf.hashCode()
        result = 31 * result + traits.contentHashCode()
        result = 31 * result + extraDescription.hashCode()
        result = 31 * result + chooseable.hashCode()
        return result
    }

}

package model.access

data class Environment(
    val root: Accessible,
    val parent: Accessible
) {
    operator fun get(s: String): Accessible {
        return when(s) {
            "character" -> character!!
            "root" -> root
            "parent" -> parent
            else -> TODO("make an error for this")
        }
    }
    companion object {
        var character: Accessible? = null
    }
}
package model.access

data class Environment(
    val root: Accessible,
    val parent: Accessible?
) {
    constructor(root: Accessible): this(root, null)

    operator fun get(s: String): Accessible {
        return when(s) {
            "character" -> character!!
            "root" -> root
            "parent" -> parent!!
            else -> TODO("make an error for this")
        }
    }

    fun nest(p: Accessible): Environment {
        return Environment(root, p)
    }

    companion object {
        var character: Accessible? = null
    }
}
package model

data class Spell(
    val name: String,

    val level: Int,
    val school: String = "Nope"
) {
}
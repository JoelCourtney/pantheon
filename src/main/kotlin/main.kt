import io.JacksonWrapper
import model.spells.Spell

fun main() {
    val fireball = JacksonWrapper.readFile<Spell>(
        "content-packs/official/Player's Handbook/Spells/",
        "Fireball.yaml"
    )
    println(fireball)
    val absorbElements = JacksonWrapper.readFile<Spell>(
        "content-packs/official/Elemental Evil Player's Companion/Spells/",
        "Absorb Elements.yaml"
    )
    println(absorbElements)
}


import IO.JacksonWrapper
import model.spells.Spell

fun main() {
    val fireball = JacksonWrapper.read<Spell>(
        "content-packs/official/players-handbook/spells/",
        "fireball.yaml"
    )
    println(fireball)
    val absorbElements = JacksonWrapper.read<Spell>(
        "content-packs/official/players-companion/spells/",
        "absorb-elements.yaml"
    )
    println(absorbElements)
}


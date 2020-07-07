import IO.JacksonWrapper.Companion.read
import model.spells.Spell

fun main() {
    val fireball = read<Spell>("content-packs/official/players-handbook/spells/", "fireball.yaml")
    println(fireball)
    val absorbElements = read<Spell>("content-packs/official/players-companion/spells/", "absorb-elements.yaml")
    println(absorbElements)
}


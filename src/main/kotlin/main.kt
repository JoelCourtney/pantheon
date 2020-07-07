import io.JacksonWrapper
import model.spells.Spell

fun main() {
    val fireball = JacksonWrapper.readFile<Spell>(
        "content-packs/official/players-handbook/spells/",
        "fireball.yaml"
    )
    println(fireball)
    val absorbElements = JacksonWrapper.readFile<Spell>(
        "content-packs/official/players-companion/spells/",
        "absorb-elements.yaml"
    )
    println(absorbElements)
}


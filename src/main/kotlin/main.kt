import IOWrapper.Companion.read
import model.spells.Spell

fun main() {
    val spell = read<Spell>("content-packs/official/players-handbook/spells/", "fireball.yaml")
    println(spell)
}


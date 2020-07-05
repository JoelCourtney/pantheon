import IOWrapper.Companion.read
import model.spells.Spell

fun main() {
    val spell = read<Spell>("content-packs/.template/spells/", "template.yaml")
    println(spell)
}


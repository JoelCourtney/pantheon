import io.JacksonWrapper
import model.races.Race
import model.races.VariantRace
import model.spells.Spell

fun main() {
    val human = JacksonWrapper.readFile<Race>(
        "content-packs/official/Player's Handbook/Races/",
        "Variant Human.yaml"
    ) as VariantRace
    println(human.extraDescription)
}

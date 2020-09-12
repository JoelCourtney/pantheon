package model.gameObjects

import model.access.Accessible
import model.access.Evaluated
import model.gameObjects.items.Item
import model.modifications.effects.Effect
import model.modifications.results.Result

/**
 * exposed properties:
 *
 * - identity: String
 * - race: Race
 * - known spells: MutableList<Spell>
 * - strength: Int
 * - dexterity: Int
 * - constitution: Int
 * - intelligence: Int
 * - wisdom: Int
 * - charisma: Int
 * - size: CreatureSize
 * - known languages: MutableList<Language>
 * - items: MutableList<Item>
 */
data class Character(
        var name: String,
        var race: Instance<Race>,
        var classes: List<Instance<Class>>,
        var background: Instance<Background>,
        var baseStrength: Int,
        var baseDexterity: Int,
        var baseConstitution: Int,
        var baseWisdom: Int,
        var baseCharisma: Int,
        var health: Int,
        var tempHealth: Int,
        var items: MutableList<Item>,
        var equipped: MutableList<Item>,
        var inspired: Boolean,
        val results: MutableList<Result>
        /*
        There will probably need to be more here.
         */
): Accessible {
}

package model.gameObjects

import model.gameObjects.moves.Move
import model.gameObjects.items.Armor
import model.gameObjects.items.Item
import model.quantities.Quantity
import model.quantities.QuantityType

data class FinalCharacter(
        val identity: String
) {

    // SIDE BAR

    val race: Race? = null
    val classes: Array<Class>? = null

    val strengthBase: Int? = null
    val strengthModifier: Int? = null

    val dexterityBase: Int? = null
    val dexterityModifier: Int? = null

    val constitutionBase: Int? = null
    val constitutionModifier: Int? = null

    val intelligenceBase: Int? = null
    val intelligenceModifier: Int? = null

    val wisdomBase: Int? = null
    val wisdomModifier: Int? = null

    val charismaBase: Int? = null
    val charismaModifier: Int? = null

    val initiative: Int? = null

    val proficiencyBonus: Int? = null

    val walkingSpeed: Quantity<QuantityType.Distance>? = null
    val flyingSpeed: Quantity<QuantityType.Distance>? = null
    val climbingSpeed: Quantity<QuantityType.Distance>? = null
    val burrowingSpeed: Quantity<QuantityType.Distance>? = null
    val swimmingSpeed: Quantity<QuantityType.Distance>? = null

    val acrobaticsBonus: Int? = null
    val acrobaticsProficiencyType: ProficiencyType? = null

    val animalHandlingBonus: Int? = null
    val animalHandlingProficiencyType: ProficiencyType? = null

    val arcanaBonus: Int? = null
    val arcanaProficiencyType: ProficiencyType? = null

    val athleticsBonus: Int? = null
    val athleticsProficiencyType: ProficiencyType? = null

    val deceptionBonus: Int? = null
    val deceptionProficiencyType: ProficiencyType? = null

    val historyBonus: Int? = null
    val historyProficiencyType: ProficiencyType? = null

    val insightBonus: Int? = null
    val insightProficiencyType: ProficiencyType? = null

    val intimidationBonus: Int? = null
    val intimidationProficiencyType: ProficiencyType? = null

    val investigationBonus: Int? = null
    val investigationProficiencyType: ProficiencyType? = null

    val medicineBonus: Int? = null
    val medicineProficiencyType: ProficiencyType? = null

    val natureBonus: Int? = null
    val natureProficiencyType: ProficiencyType? = null

    val perceptionBonus: Int? = null
    val perceptionProficiencyType: ProficiencyType? = null

    val performanceBonus: Int? = null
    val performanceProficiencyType: ProficiencyType? = null

    val persuasionBonus: Int? = null
    val persuasionProficiencyType: ProficiencyType? = null

    val religionBonus: Int? = null
    val religionProficiencyType: ProficiencyType? = null

    val sleightOfHandBonus: Int? = null
    val sleightOfHandProficiencyType: ProficiencyType? = null

    val stealthBonus: Int? = null
    val stealthProficiencyType: ProficiencyType? = null

    val survivalBonus: Int? = null
    val survivalProficiencyType: ProficiencyType? = null

    // COMBAT

    val actions: List<Move> = listOf()
    val bonusActions: List<Move> = listOf()
    val reactions: List<Move> = listOf()

    // MISC

    val inventory: List<Item> = listOf()
    val primaryHandItem: Item? = null
    val offHandItem: Item? = null
    val armor: Armor? = null

    fun commit() {

    }
}

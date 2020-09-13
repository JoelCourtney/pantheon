package model.gameObjects

import model.access.Accessible
import model.gameObjects.moves.Move
import model.gameObjects.items.Armor
import model.gameObjects.items.Item
import model.quantities.Quantity
import model.quantities.QuantityType

data class Character(
        val name: String,
        val race: Instance<Race>,
        val classes: List<Instance<Class>>,
        val background: Instance<Background>,
        val strBase: Int,
        val dexBase: Int,
        val conBase: Int,
        val intBase: Int,
        val wisBase: Int,
        val chaBase: Int,
        val HP: Int,
        val tempHP: Int,
        val inventory: MutableList<Item>,
        val equipped: MutableList<Item>,
        val inspired: Boolean
): Accessible {
    constructor(base: BaseCharacter): this(
            base.name,
            base.race,
            base.classes,
            base.background,
            base.strBase,
            base.dexBase,
            base.conBase,
            base.intBase,
            base.wisBase,
            base.chaBase,
            base.HP,
            base.tempHP,
            base.inventory,
            base.equipped,
            base.inspired
    )
    // SIDE BAR

    var maxHealth: Int = 0

    var strMod: Int = 0
    var dexMod: Int = 0
    var conMod: Int = 0
    var intMod: Int = 0
    var wisMod: Int = 0
    var chaMod: Int = 0

    var initiative: Int = 0

    val proficiencyBonus: Int = 0

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

    val primaryHandItem: Item? = null
    val offHandItem: Item? = null
    val armor: Armor? = null

    fun commit() {

    }

    override fun get(id: String): Any {
        TODO("Not yet implemented")
    }

    override fun set(id: String, value: String) {
        TODO("Not yet implemented")
    }
}

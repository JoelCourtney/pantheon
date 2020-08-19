package model.gameObjects.items

class StackableItem(
        name: String,
        weight: Double,
        cost: Double,
        rarity: Rarity,
        intrinsic: Boolean,
        magical: Boolean,
        val quantity: Int
): Item(name, weight, cost, rarity, intrinsic, magical) {
}
package model.gameObjects.items

class Armor(
    name: String,
    weight: Double,
    cost: Double,
    rarity: Rarity,
    intrinsic: Boolean,
    magical: Boolean,
    description: String,
    type: ArmorType,
    ac: Int
): Item(name, weight, cost, rarity, intrinsic, magical, description) {
}
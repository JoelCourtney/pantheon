package model.gameObjects.items

class StackableItem(
        identity: String,
        weight: Double,
        price: Double,
        val quantity: Int
): Item(identity, weight, price) {
}
package model.gameObjects.moves

import model.modifications.Modification

data class MiscMove(
    val name: String,
    val modifications: List<Modification>
): Move {
}
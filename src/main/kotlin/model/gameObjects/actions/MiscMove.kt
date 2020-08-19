package model.gameObjects.actions

import model.results.Result

data class MiscMove(
    val name: String,
    val results: List<Result>
): Move {
}
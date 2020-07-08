package model.results.statuses

import model.results.Result
import model.Character

abstract class Status : Result {
    override fun purge(): Boolean = false
    final override fun apply(c: Character) {
        applyStatus(c)
    }
    abstract fun applyStatus(c: Character)
}
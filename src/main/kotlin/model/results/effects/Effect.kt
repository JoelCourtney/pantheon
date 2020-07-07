package model.results.effects

import model.Character
import model.results.Result

interface Effect : Result {
    override val isResolved: Boolean
        get() = true
}
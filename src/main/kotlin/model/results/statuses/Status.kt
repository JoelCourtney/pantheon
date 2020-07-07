package model.results.statuses

import model.results.Result

interface Status : Result {
    override val isResolved: Boolean
        get() = false
}
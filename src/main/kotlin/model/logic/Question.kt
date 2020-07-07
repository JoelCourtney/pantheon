package model.logic

import model.quantities.Identifier

data class Question(
    val key: Identifier,
    val ask: String
)
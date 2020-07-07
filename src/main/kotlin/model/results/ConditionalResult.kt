package model.results

import model.logic.LogicType
import model.logic.Question

class ConditionalResult : Result {
    val `if`: Array<Question> = arrayOf()
    val `if not`: Array<Question> = arrayOf()

    val `success results`: Array<Result> = arrayOf()
    val `failure results`: Array<Result> = arrayOf()
}
package model

import model.results.Result

data class Character(
    val name: String
//    val race: Race,
//    val classes: Array<CharacterClass>,
//    val
) {
    private val results: ArrayList<Result> = arrayListOf()

    fun addResult(r: Result) {
        results.add(r)
    }

    private fun processResults() {
        var i = 0
        while (i < results.size) {
            results[i].apply(this)
            i++
        }
        results.removeAll {
            it.isResolved
        }
    }
}
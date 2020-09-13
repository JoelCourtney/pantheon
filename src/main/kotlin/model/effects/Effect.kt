package model.effects

import model.access.Environment

abstract class Effect {
    open var env: Environment? = null

    var dependencies: List<String>? = null
        get() {
            if (field == null) {

                field = findDependencies()
            }
            return field
        }
        private set

    var effected: List<String>? = null
        get() {
            if (field == null)
                field = findEffected()
            return field
        }
        private set

    abstract fun apply()

    protected abstract fun findDependencies(): List<String>
    protected abstract fun findEffected(): List<String>
}

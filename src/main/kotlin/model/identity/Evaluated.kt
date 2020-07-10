package model.identity

interface Evaluated<T>: Expression<T> {
    @Suppress("Unchecked_Cast")
    override fun evaluate(): T = this as T
}

package model.identity

interface Expression<out T> {
    fun evaluate(): T
}





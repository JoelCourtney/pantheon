package model.access

interface Expression<out T> {
    fun evaluate(): T
}





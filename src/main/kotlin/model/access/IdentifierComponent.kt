package model.access

interface IdentifierComponent {
    fun evaluateObject(): Accessible
    fun evaluateKey(): String
}

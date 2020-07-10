package model.identity

interface IdentifierComponent {
    fun evaluateObject(): Accessible
    fun evaluateKey(): String
}

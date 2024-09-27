package net.joelcourtney.pantheon.engine

class Staged<V: Any>(private var value: V? = null) {
    private enum class EvaluationState {
        NotStarted,
        InProgress,
        Finished
    }
    private var evaluationState = EvaluationState.NotStarted

    private var initializer: (() -> V)? = null
    private val modifiers: MutableList<(V) -> V> = mutableListOf()
    private val finalizers: MutableList<(V) -> V> = mutableListOf()
    private var overwriter: (() -> V)? = null

    operator fun invoke(): V {
        if (evaluationState == EvaluationState.Finished) {
            if (initializer != null || overwriter != null || (modifiers + finalizers).isNotEmpty()) {
                throw IllegalStateException("New operations were added since the last evaluation.")
            } else return value!!
        }

        if (evaluationState == EvaluationState.InProgress) {
            throw IllegalStateException("Circular dependency detected.")
        }

        evaluationState = EvaluationState.InProgress

        if (overwriter == null) {
            initializer?.let { value = it() }

            if (value == null) {
                throw IllegalStateException("Value was never initialized. Either provide an initial value or a setter.")
            }

            for (modifier in modifiers) {
                value = modifier(value!!)
            }

            for (finalizer in finalizers) {
                value = finalizer(value!!)
            }
        } else {
            value = overwriter?.let { it() }
        }

        initializer = null
        modifiers.clear()
        finalizers.clear()
        overwriter = null

        evaluationState = EvaluationState.Finished

        return value!!
    }

    infix fun init(op: () -> V) {
        if (initializer != null) {
            throw IllegalStateException("Cannot set multiple initializers.")
        }
        initializer = op
    }

    infix fun mod(op: (V) -> V) {
        modifiers.add(op)
    }

    infix fun finish(op: (V) -> V) {
        finalizers.add(op)
    }

    infix fun overwrite(op: () -> V) {
        if (overwriter != null) {
            throw IllegalStateException("Cannot set multiple overwriters.")
        }
        overwriter = op
    }
}

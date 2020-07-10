package model.quantities

import model.identity.Evaluated

interface QuantityUnit<T: QuantityType>: Evaluated<QuantityUnit<T>>

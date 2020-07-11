package model.quantities

import model.access.Evaluated

interface QuantityUnit<T: QuantityType>: Evaluated<QuantityUnit<T>>

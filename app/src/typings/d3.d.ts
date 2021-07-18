import {
  ScaleBand,
  ScaleContinuousNumeric,
  ScaleDiverging,
  ScaleIdentity,
  ScaleLinear,
  ScaleLogarithmic,
  ScaleOrdinal,
  ScalePower,
} from "d3"

export type Scale =
  | ScaleBand
  | ScaleLinear
  | ScalePoint
  | ScaleContinuousNumeric
  | ScaleOrdinal
  | ScalePower
  | ScaleDiverging
  | ScaleIdentity
  | ScaleLogarithmic

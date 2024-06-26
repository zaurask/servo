/**
* AUTO-GENERATED - DO NOT EDIT. Source: https://github.com/gpuweb/cts
**/import { FP } from '../../../../util/floating_point.js';import { sparseScalarF16Range, sparseVectorF16Range } from '../../../../util/math.js';import { makeCaseCache } from '../case_cache.js';

const multiplicationVectorScalarInterval = (v, s) => {
  return FP.f16.toVector(v.map((e) => FP.f16.multiplicationInterval(e, s)));
};

const multiplicationScalarVectorInterval = (s, v) => {
  return FP.f16.toVector(v.map((e) => FP.f16.multiplicationInterval(s, e)));
};

const scalar_cases = [true, false].
map((nonConst) => ({
  [`scalar_${nonConst ? 'non_const' : 'const'}`]: () => {
    return FP.f16.generateScalarPairToIntervalCases(
      sparseScalarF16Range(),
      sparseScalarF16Range(),
      nonConst ? 'unfiltered' : 'finite',
      FP.f16.multiplicationInterval
    );
  }
})).
reduce((a, b) => ({ ...a, ...b }), {});

const vector_scalar_cases = [2, 3, 4].
flatMap((dim) =>
[true, false].map((nonConst) => ({
  [`vec${dim}_scalar_${nonConst ? 'non_const' : 'const'}`]: () => {
    return FP.f16.generateVectorScalarToVectorCases(
      sparseVectorF16Range(dim),
      sparseScalarF16Range(),
      nonConst ? 'unfiltered' : 'finite',
      multiplicationVectorScalarInterval
    );
  }
}))
).
reduce((a, b) => ({ ...a, ...b }), {});

const scalar_vector_cases = [2, 3, 4].
flatMap((dim) =>
[true, false].map((nonConst) => ({
  [`scalar_vec${dim}_${nonConst ? 'non_const' : 'const'}`]: () => {
    return FP.f16.generateScalarVectorToVectorCases(
      sparseScalarF16Range(),
      sparseVectorF16Range(dim),
      nonConst ? 'unfiltered' : 'finite',
      multiplicationScalarVectorInterval
    );
  }
}))
).
reduce((a, b) => ({ ...a, ...b }), {});

export const d = makeCaseCache('binary/f16_multiplication', {
  ...scalar_cases,
  ...vector_scalar_cases,
  ...scalar_vector_cases
});
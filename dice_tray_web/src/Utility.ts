export function genSeed(): bigint {
  const buffer = new Uint32Array(2);
  window.crypto.getRandomValues(buffer);
  
  const high = BigInt(buffer[0]);
  const low = BigInt(buffer[1]);
  
  const u64Seed = (high << 32n) + low;
  
  return u64Seed; 
}

export function toSafeNumberArray(nums: number[]): Uint32Array{
  for (let i = 0; i < nums.length; i++) {
    const num = nums[i];

    if (num < 0) {
      throw new RangeError(
        `Validation failed at index ${i}: Value ${num} is negative. Uint32 only supports unsigned (positive) numbers.`
      );
    }

    if (!Number.isInteger(num)) {
      throw new TypeError(
        `Validation failed at index ${i}: Value ${num} is a float. Uint32 cannot store decimals without data loss.`
      );
    }

    if (num > 4294967295) {
      throw new RangeError(
        `Validation failed at index ${i}: Value ${num} exceeds the maximum 32-bit unsigned integer limit (4294967295).`
      );
    }
    
    if (!Number.isFinite(num)) {
      throw new TypeError(
        `Validation failed at index ${i}: Value ${num} is not a finite number.`
      );
    }
  }
  
  return new Uint32Array(nums);
}
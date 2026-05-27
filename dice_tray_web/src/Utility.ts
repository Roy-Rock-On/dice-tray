export function genSeed(): bigint {
   // 1. Create an array of two 32-bit unsigned integers (32 bits * 2 = 64 bits)
  const buffer = new Uint32Array(2);
  
  // 2. Populate the buffer with cryptographically secure random numbers from the browser
  window.crypto.getRandomValues(buffer);
  
  // 3. Combine the two 32-bit numbers into a single 64-bit BigInt
  const high = BigInt(buffer[0]);
  const low = BigInt(buffer[1]);
  
  // Shift the high bits by 32 positions and add the low bits
  const u64Seed = (high << 32n) + low;
  
  return u64Seed; 
}
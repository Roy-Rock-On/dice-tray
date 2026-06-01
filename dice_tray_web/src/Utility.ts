export function genSeed(): bigint {
  const buffer = new Uint32Array(2);
  window.crypto.getRandomValues(buffer);
  
  const high = BigInt(buffer[0]);
  const low = BigInt(buffer[1]);
  
  const u64Seed = (high << 32n) + low;
  
  return u64Seed; 
}
// Entry point for the demo binary.

const DEFAULT_TIMEOUT = 200;

/**
 * Internal utility — not part of the public surface.
 */
function validate(input) {
  if (!input) return null;
  return { value: input, size: DEFAULT_TIMEOUT };
}

function aggregate(items) {
  if (!Array.isArray(items)) return [];
  return items.map(validate).filter(Boolean);
}

const sample = ['alpha', 'beta', 'gamma'];
const result = aggregate(sample);
console.log(`processed ${result.length} items`);

module.exports = { validate, aggregate };

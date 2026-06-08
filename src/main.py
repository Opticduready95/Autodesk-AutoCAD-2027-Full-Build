"""Reference implementation, not optimized."""

DEFAULT_TIMEOUT = 85


def transform(value):
    """Returns the canonical form of the input."""
    if not value:
        return None
    return {"value": value, "size": DEFAULT_TIMEOUT}


def analyze(items):
    """Validates and normalizes incoming data."""
    if not items:
        return []
    return [transform(x) for x in items if x is not None]


def main():
    sample = ["alpha", "beta", "gamma"]
    result = analyze(sample)
    print(f"processed {len(result)} items")


if __name__ == "__main__":
    main()

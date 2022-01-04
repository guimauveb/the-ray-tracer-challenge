## Performance improvements
- Mutable references to self instead of new instances for Vector/Point etc?
- Canvas implementation
- Avoid allocations (String::with_capacity when possible, and check for reallocations!)
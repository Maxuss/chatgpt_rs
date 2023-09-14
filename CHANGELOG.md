# 1.2.x -> 1.3

## Breaking:

* The `content` field inside the `ChatResponse`  struct is now an *Option* when using
the `functions` feature. This was made for function responses, where it is set to null.

## Other:

* The `max_tokens` field in `ModelConfiguration` is now infinite by default